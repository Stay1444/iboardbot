// iBoardBot PROJECT Arduino M0 version
// Author: Jose Julio & Juan pedro (JJROBOTS)
// Hardware: New JJROBOTS DEVIA Board with M0 microcontroller
// Board: Arduino/Genuine Zero (Native USB Port)
// ESP8266 firmware version 1.0.1 or higher
// Date: 18/04/2015
// Last updated: 28/10/2019
// Version: 1.14
// Project page : http://jjrobots.com/
// Kickstarter page:https://www.kickstarter.com/projects/879074320/iboardbot-the-internet-controlled-whiteboard-robot
// GIT repository: https://github.com/jjrobots/iBoardbot
// License: Open Software GPL License v2

// Hardware:
// X Motor (longitudinal move) connected to MOTOR2 output
// Y Motor (vertical move) connected to MOTOR1 output
// Eraser servo connected to Servo1 output
// PEN Lift servo connected to Servo2 output

// NO EEPROM ON M0!!! => We need FlasStorage library (you could install it from arduino library manager)
#include <FlashAsEEPROM.h>
//#include <FlashStorage.h>

#define VERSION "iBoardBot 1.14"
//#define DEBUG 0

// ROBOT and USER configuration parameters
#include "Configuration.h"
#include <Servo.h>

Servo servo1; 
Servo servo2; 

unsigned char buffer[MAX_PACKET_SIZE];   // buffer to store incomming message from server
bool draw_task = false;
bool new_packet = false;
int block_number;
bool next_block = false;
char MAC[13];  // MAC address of Wifi module

// Return board switch state
bool board_switch_pressed()
{
  return !(REG_PORT_IN1 & PORT_PB03);
}

// Configuration: Pins, servos, Steppers, Wifi...
void setup()
{
  // STEPPER PINS ON JJROBOTS BROBOT BRAIN BOARD
  // STEPPER PINS ON JJROBOTS DEVIA M0 BOARD
  pinMode(11, OUTPUT); // ENABLE MOTORS   ATSAMD21:PA16
  pinMode(5, OUTPUT); // STEP MOTOR 1 ATSAMD21:PA15
  pinMode(6, OUTPUT); // DIR MOTOR 1  ATSAMD21:PA20
  pinMode(7, OUTPUT); // STEP MOTOR 2 ATSAMD21:PA21
  pinMode(8, OUTPUT); // DIR MOTOR 2  ATSAMD21:PA06
  pinMode(9, OUTPUT); // STEP MOTOR 3 ATSAMD21:PA07
  pinMode(10, OUTPUT); // DIR MOTOR 3  ATSAMD21:PA18

  pinMode(A4, OUTPUT);    // Microstepping output
  digitalWrite(A4, HIGH); // 1/16 (default config)

  pinMode(RED_LED, OUTPUT); // RED LED
  pinMode(GREEN_LED, OUTPUT); // GREEN LED
  pinMode(SWITCH_IN, INPUT_PULLUP);  // Input Switch
  digitalWrite(SWITCH_IN,OUTPUT); // PULLUP

  pinMode(3, OUTPUT); // SERVOS
  pinMode(4, OUTPUT); //

  digitalWrite(11, HIGH);  // Disbale stepper motors 

  digitalWrite(RED_LED, HIGH);
  digitalWrite(GREEN_LED, LOW);

  delay(100);
  SerialUSB.begin(115200); // Serial output to console

  Serial1.begin(115200); // Wifi initialization

  delay(2000);

  // Manual Configuration mode? => Erase stored Wifi parameters (force A0 to ground)
  if (board_switch_pressed())
    writeWifiConfig(0, "", "", "", 0, "");

#ifdef DEBUG
  delay(10000);  // Only needed for serial debug
  SerialUSB.println(VERSION);
#endif

  // Init servos
  SerialUSB.println("Servo init...");
  servo_pos1 = SERVO1_ERASER;
  servo_pos2 = SERVO2_PAINT;
  initServo();
  moveServo1(SERVO1_ERASER);
  moveServo2(SERVO2_PAINT);
  delay(1000);
  disableServo1();
  disableServo2();
  
  SerialUSB.println("Reading Wifi configuration...");
  readWifiConfig();

  SerialUSB.println(WifiConfig.status);
  SerialUSB.println(WifiConfig.ssid);
  //SerialUSB.println(WifiConfig.pass);
  SerialUSB.println("****");
  SerialUSB.println(WifiConfig.api_ip);
  SerialUSB.println(WifiConfig.api_port);
  SerialUSB.println(WifiConfig.board_name);

  // if wifi parameters are not configured we start the config web server
  if (WifiConfig.status != 1) {
    WifiConfigurationMode();
  }

  Wconnected = false;
  Network_errors = 0;
  while (!Wconnected) {
    // Wifi initialization
    Serial1.println();
    ESPflush();
    Serial1.println("AT+RST");
    delay(1000);
    Serial1.println("AT+GMR");
    delay(4000);
    GetMac();

    if (WifiConnect()) {
      digitalWrite(GREEN_LED, HIGH);
      Wconnected = true;
    }
    else {
      SerialUSB.println("Error connecting to network!");
      digitalWrite(GREEN_LED, LOW);
      delay(2500);
      Wconnected = false;
      Network_errors++;
      // If connection fail 2 times we enter WifiConfigurationMode
      if (Network_errors >= 2) {
        WifiConfigurationMode();
        Network_errors = 0;
      }
    }
  }

  // Configure TCP client (for HTTP connection to server)
  delay(250);
  Serial1.println("AT+CIFSR");
  ESPwaitFor("OK", 3);
  Serial1.println("AT+CIPMUX=0");
  ESPwaitFor("OK", 3);
  Serial1.println("AT+CIPMODE=0");
  ESPwaitFor("OK", 3);
  Network_errors = 0;
  delay(200);

  //Initializing init position
  position_x = ROBOT_INITIAL_POSITION_X * X_AXIS_STEPS_PER_UNIT;
  position_y = ROBOT_INITIAL_POSITION_Y * Y_AXIS_STEPS_PER_UNIT;

  // Output parameters
  //SerialUSB.print("Max_acceleration_x: ");
  //SerialUSB.println(acceleration_x);
  //SerialUSB.print("Max_acceleration_y: ");
  //SerialUSB.println(acceleration_y);
  //SerialUSB.print("Max speed X: ");
  //SerialUSB.println(MAX_SPEED_X);
  //SerialUSB.print("Max speed Y: ");
  //SerialUSB.println(MAX_SPEED_Y);

  // STEPPER MOTORS INITIALIZATION
  SerialUSB.println("Steper motors initialization...");

  timersConfigure();
  SerialUSB.println("Timers initialized");
  timersStart(); //starts the timers
  SerialUSB.println("Timers started");
  
  delay(200);
  
  SerialUSB.println("Moving to initial position...");
  // Initializing Robot command variables
  com_speed_x = MAX_SPEED_X / 2;
  com_speed_y = MAX_SPEED_Y / 2;
  max_speed_x = com_speed_x;
  max_speed_y = com_speed_y;

  //Initializing init position
  position_x = ROBOT_INITIAL_POSITION_X * X_AXIS_STEPS_PER_UNIT;
  position_y = ROBOT_INITIAL_POSITION_Y * Y_AXIS_STEPS_PER_UNIT;
  //last_position_x = position_x;
  //last_position_y = position_y;

  setSpeedS(com_speed_x, com_speed_y);
  setPosition_mm10(ROBOT_MIN_X * 10, ROBOT_MIN_Y * 10);
  last_move_x = ROBOT_INITIAL_POSITION_X * 10;
  last_move_y = ROBOT_INITIAL_POSITION_Y * 10;
  home_position = true;

  SerialUSB.println("Initial position configured!");
  delay(50);

  // Enable TIMERs interrupts
  //TIMSK1 |= (1 << OCIE1A); // Enable Timer1 interrupt
  //TIMSK3 |= (1 << OCIE1A); // Enable Timer1 interrupt
  
  delay(50);

  ESPflush();

  SerialUSB.println("API parameters:");
  SerialUSB.print("Host:");
  SerialUSB.println(WifiConfig.api_ip);
  SerialUSB.print("Port:");
  SerialUSB.println(WifiConfig.api_port);
  SerialUSB.print("Board: ");
  SerialUSB.println(WifiConfig.board_name);

  SerialUSB.println();
  SerialUSB.println(VERSION);
  //SerialUSB.print("ID_IWBB ");
  SerialUSB.println(MAC);
  SerialUSB.println(" Ready...");
  timer_old = micros();
  timeout_counter = 0;
  block_number = -1;
  commands_index = 0;
  commands_lines = 0;
  draw_task = false;

  // Ready signal
  enableServo1();
  enableServo2();
  moveServo2(SERVO2_PAINT - 200);
  delay(300);
  moveServo2(SERVO2_PAINT);
  delay(300);
  moveServo2(SERVO2_PAINT - 200);
  delay(300);
  moveServo2(SERVO2_PAINT);
  delay(1000);
  if (Wconnected) {
    moveServo2(SERVO2_PAINT - 200);
    delay(300);
    moveServo2(SERVO2_PAINT);
    delay(300);
  }

  disableServo1();
  disableServo2();
  
  digitalWrite(RED_LED, LOW);
}


void loop()
{
  uint8_t response;
  char get_string[110];
  int code1;
  int code2;

  debug_counter++;
  timer_value = micros();
  dt = timer_value - timer_old;
  if (dt >= 1000) { // 1Khz loop
    timer_old = timer_value;
    loop_counter++;
    if (draw_task) {
      adjustSpeed();
      positionControl(dt);   // position, speed and acceleration control of stepper motors
      timeout_counter++;
      wait_counter = 0;
      if ((timeout_counter > 8000) || ((myAbsLong(target_position_x - position_x) < POSITION_TOLERANCE_X) && (myAbsLong(target_position_y - position_y) < POSITION_TOLERANCE_Y))) { // Move done?
        if (timeout_counter > 8000) {
          SerialUSB.print("!TimeoutCounter! "); // 8 seconds timeout
          show_command = true;
          // Reset position on timeout?
        }
#if DEBUG==2
        if (show_command) {
          SerialUSB.print(" EP:");
          SerialUSB.print(position_x);
          SerialUSB.print(":");
          SerialUSB.println(position_y);
        }
#endif
        if (commands_index < commands_lines) {
          // Reading bytes from Packet (3 bytes per command)
          int byte1 = buffer[commands_index * 3];
          int byte2 = buffer[commands_index * 3 + 1];
          int byte3 = buffer[commands_index * 3 + 2];
          // Decode the command (code1 and code2) each code is 12 bits
          code1 = (byte1 << 4) | ((byte2 & 0xF0) >> 4);
          code2 = ((byte2 & 0x0F) << 8) | (byte3);

#ifdef DEBUG
          if (show_command) {
            SerialUSB.print(commands_index);
            SerialUSB.print(":");
            SerialUSB.print(code1);
            SerialUSB.print(",");
            SerialUSB.print(code2);
            show_command = false;
          }
#endif
          // DECODE protocol codes
          if ((new_packet) && (code1 != 4009)) { // check new packet
            // PACKET ERROR: No valid first command!
            SerialUSB.print(" !PACKET ERROR!");
            commands_index = 0;
            draw_task = false;
            disableServo1();
            disableServo2();
            servo_counter = 0;
            dir_x = 0;
            dir_y = 0;
          }
          else if (code1 == 4009) {
            new_packet = false;
            block_number = code2;
            if (block_number >= 4000)
              block_number = -1;
            else {
              SerialUSB.print(" Start block:");
              SerialUSB.println(block_number);
            }
            show_command = true;
            servo_counter = 0;
            if (timeout_recover) {   // Timeout recovery mode? This means we had a timeout
              SerialUSB.print("->Timeout recover!");
              // Rewrite a PEN LIFT COMMAND 4003 0
              buffer[commands_index * 3] = (4003 >> 4);
              buffer[commands_index * 3+1]= ((4003 << 4) & 0xF0);
              buffer[commands_index * 3+2]= 0;
              timeout_recover = false;
              }
            else
              commands_index++;
          }
          else if ((code1 == 4001) && (code2 == 4001)) { // START DRAWING
            if (servo_counter == 0) {
              SerialUSB.println(" !START DRAW");
              enableServo1();
              enableServo2();
            }
            // Pen lift
            moveServo1(SERVO1_PAINT);
            moveServo2(SERVO2_LIFT);
            servo_counter++;
            if (servo_counter > 100) {
              digitalWrite(11, LOW); // Enable motors...
              // Default move speed
              max_speed_x = MAX_SPEED_X;
              max_speed_y = MAX_SPEED_Y;
              erase_mode = 0;
              commands_index++;
              show_command = true;
              timeout_counter = 0;
              servo_counter = 0;
            }
          }
          else if (code1 == 4002) { // END DRAWING
            SerialUSB.print(" !STOP DRAW time:");
            SerialUSB.println(millis() - draw_init_time);
            // Pen lift
            if (!servo1_ready) {
              enableServo1();
              //SerialUSB.println("EnableS1");
            }
            if (!servo2_ready) {
              enableServo2();
              //SerialUSB.println("EnableS2");
            }
            moveServo1(SERVO1_ERASER);
            moveServo2(SERVO2_PAINT);
            digitalWrite(11, HIGH); // Disable motors...
            dir_x = 0;
            dir_y = 0;
            erase_mode = 0;
            draw_task = false;
            commands_index = 0;
            servo_counter = 0;
            //delay(300);    // Nothing to do ??
            poll_again = true;
            if (code2 == 4010) {  // Special code? => timeout_recovery on next block
              timeout_recover = true;
              SerialUSB.print("->Timeout recover mode!");
            }
            else
              timeout_recover = false;
            //stopServo();
            disableServo1();
            disableServo2();
          }
          else if (code1 == 4003) { // Pen lift command
            if (!servo1_ready) {
              enableServo1();
            }
            if (!servo2_ready) {
              enableServo2();
            }
            moveServo1(SERVO1_PAINT);
            moveServo2(SERVO2_LIFT);
            servo_counter++;
            if (servo_counter > 90) {
              erase_mode = 0;
              commands_index++;
              show_command = true;
              max_speed_x = MAX_SPEED_X;
              max_speed_y = MAX_SPEED_Y;
              SerialUSB.println(" SNP");
              timeout_counter = 0;
              servo_counter = 0;
              disableServo2();
            }
          }
          // Pen down command
          else if (code1 == 4004) {
            if (!servo1_ready) {
              enableServo1();
            }
            if (!servo2_ready) {
              enableServo2();
            }
            moveServo1(SERVO1_PAINT);
            moveServo2(SERVO2_PAINT);
            servo_counter++;
            //SerialUSB.println(servo_pos);
            if (servo_counter > 180) {
              servo_pos2 = SERVO2_PAINT;
              erase_mode = 0;
              commands_index++;
              show_command = true;
              max_speed_x = SPEED_PAINT_X;
              max_speed_y = SPEED_PAINT_Y;
              SerialUSB.println(" SP");
              timeout_counter = 0;
              servo_counter = 0;
              disableServo2();
            }
          }
          // Eraser command
          else if (code1 == 4005) {
            if (!servo1_ready) {
              enableServo1();
            }
            if (!servo2_ready) {
              enableServo2();
            }
            // Make position correction for eraser
            setPosition_mm10(last_move_x + ERASER_OFFSET_X * 10, last_move_y + ERASER_OFFSET_Y * 10);
            moveServo1(SERVO1_ERASER);
            moveServo2(SERVO2_PAINT);
            servo_counter++;
            //moveServo2(SERVO2_PAINT);
            //SerialUSB.println(servo_pos);
            if (servo_counter > 350) {
              servo_pos1 = SERVO1_ERASER;
              erase_mode = 1;
              commands_index++;
              show_command = true;
              max_speed_x = SPEED_ERASER_X;
              max_speed_y = SPEED_ERASER_Y;
              SerialUSB.println(" SE");
              timeout_counter = 0;
              servo_counter = 0;
              disableServo1();
              disableServo2();
            }
          }
          // Wait command
          else if (code1 == 4006) {
            disableServo1();
            disableServo2();
            delay_counter++;
            if (code2 > 30) // maximun 30 seconds of wait
              code2 = 30;
            if (delay_counter > ((long)code2 * 1000)) { // Wait for code2 seconds
              SerialUSB.println(" WM");
              commands_index++;
              show_command = true;
              delay_counter = 0;
              timeout_counter = 0;
            }
          }
          else {
            if (servo1_ready)
              disableServo1();
            if (servo2_ready)
              disableServo2();
            timeout_counter = 0;
            // Send coordinates to robot in mm/10 units
            if (erase_mode == 1) { // In erase mode, we make the correction of the position of the eraser
              setPosition_mm10(code1 + ERASER_OFFSET_X * 10, code2 + ERASER_OFFSET_Y * 10);
              SerialUSB.print("E");
            }
            else {
              if ((code1 < 10) && (code2 < 10)) { // Home position?
                setPosition_mm10(code1, code2);
                home_position = true;
              }
              else {
                setPosition_mm10(code1 + PAINT_OFFSET_X * 10, code2 + PAINT_OFFSET_Y * 10);
                home_position = false;
              }
            }
            last_move_x = code1;
            last_move_y = code2;
#if DEBUG>0
            SerialUSB.print("   ");
            SerialUSB.print(target_position_x);
            SerialUSB.print(",");
            SerialUSB.print(target_position_y);
#endif

#ifdef DEBUG
            SerialUSB.println();
#endif
            commands_index++;
            show_command = true;
          }

        }
        else {
          // End of commands...
          SerialUSB.print(" !FINISH! time:");
          SerialUSB.println(millis() - draw_init_time);
          commands_index = 0;
          draw_task = false;
          disableServo1();
          disableServo2();
          servo_counter = 0;
          dir_x = 0;
          dir_y = 0;
        }
      }

#if DEBUG==2
      if ((loop_counter % 50) == 0) {
        SerialUSB.print(position_x);
        SerialUSB.print(":");
        SerialUSB.print(position_y);
        SerialUSB.println();
      }
#endif
    }  // draw task
    else {
      // Polling server, task for me?
      loop_counter = 0;
      poll_again = false;
      if (!home_position) {
        if (wait_counter == 0)
          wait_counter = millis();
        if ((millis() - wait_counter) > 8000) {
          // Force Go to home command
          SerialUSB.println("->Force Home!");
          draw_task = true;
          new_packet = true;
          digitalWrite(11, LOW); // Enable motors...
          show_command = true;
          timeout_counter = 0;
          // ($code1>>4),(($code1<<4)&0xF0)|(($code2>>8)&0x0F),($code2&0xFF)
          // 4009,4000
          buffer[0] = (4009 >> 4);
          buffer[1] = ((4009 << 4) & 0xF0) | ((4000 >> 8) & 0x0F);
          buffer[2] = (4000 & 0xFF);
          // 4001,4001
          buffer[3] = (4001 >> 4);
          buffer[4] = ((4001 << 4) & 0xF0) | ((4001 >> 8) & 0x0F);
          buffer[5] = (4001 & 0xFF);
          // 4003,0000
          buffer[6] = (4003 >> 4);
          buffer[7] = ((4003 << 4) & 0xF0) | ((0 >> 8) & 0x0F);
          buffer[8] = 0;
          // 1,1
          buffer[9]  = 0;
          buffer[10] = ((1 << 4) & 0xF0) | 0;
          buffer[11] = 1;
          // 4002,4002
          buffer[12] = (4002 >> 4);
          buffer[13] = ((4002 << 4) & 0xF0) | ((4010 >> 8) & 0x0F);  // Special END 4002 4010 => timeout recover
          buffer[14] = (4010 & 0xFF);
          commands_lines = 5;
          draw_init_time = millis();
          return; // End
        }
      } // if(!home_position)
      //delay(20);
      SerialUSB.println();
      SerialUSB.println("POLL server...");
      ESPflush();
      //ESPwait(1);
      if (block_number == -1) {
        // Ready for new blocks...
        sprintf(get_string, "%s%s:%d%s%s%s", "http://", WifiConfig.api_ip, WifiConfig.api_port, "/_/board/", WifiConfig.board_name, "?STATUS=READY");
        response = ESPsendHTTP(get_string);
      }
      else {
        // ACK last block and ready for new one...
        sprintf(get_string, "%s%s:%d%s%s%s%d", "http://", WifiConfig.api_ip, WifiConfig.api_port, "/_/board/", WifiConfig.board_name, "?STATUS=ACK&NUM=", block_number);
        response = ESPsendHTTP(get_string);
      }
      if (response) {
        message_readed = 0;
        message_status = 0;
        message_timer_init = millis();
        while (message_readed == 0)
          ESPwaitforMessage(30);
        if (message_readed == 2) {
          SerialUSB.println("!ERROR: M Timeout");
          Network_errors++;
          // if we have three or more consecutive Network errors we reset wifi connection
          if (Network_errors >= 3) {
            SerialUSB.print("Too much network errors: Reseting...");
            Network_errors = 0;
            digitalWrite(GREEN_LED, LOW);
            // Reset module
            Serial1.println();
            ESPflush();
            Serial1.println("AT+RST");
            ESPwaitFor("ready", 10);
            if (WifiConnect()) {
              digitalWrite(GREEN_LED, HIGH);
            }
            else {
              SerialUSB.print("Error trying to reconnect!");
              digitalWrite(GREEN_LED, LOW);
            }
          }
        }
        else if (message_size < 6) {
          Network_errors = 0;
          SerialUSB.print("Data size:");
          SerialUSB.println(message_size);
          if (message_size > 0) {
            SerialUSB.print("Message:");
            SerialUSB.print((char)buffer[0]);
            SerialUSB.println((char)buffer[1]);
          }
          block_number = -1;
          // Error message?
          if (buffer[0] == 'E') {
            SerialUSB.println("Error Message!");
            SerialUSB.println("Wait for next request...");
            delay(10000);
            // Something more here??
          }
          //delay(100);
        }
        else {
          Network_errors = 0;
          SerialUSB.print("Msize:");
          SerialUSB.println(message_size);
          SerialUSB.print("Processing packet...lines:");
          commands_lines = message_size / 3;
          SerialUSB.println(commands_lines);
          commands_index = 0;
          if (message_size == MAX_PACKET_SIZE)
            SerialUSB.println("MORE BLOCKS!");
          draw_task = true;
          new_packet = true;
          digitalWrite(11, LOW); // Enable motors...
          show_command = true;
          timeout_counter = 0;
          draw_init_time = millis();
        }
      } // if (response)
    } // draw task
  } // 1Khz loop
}
