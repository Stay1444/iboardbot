// iBoardbot project

// Init servo on T4 timer. Output OC4B (Leonardo Pin10) servo1, OC4D (Leonardo Pin6) servo2, optional I2C output for servo2
// Servo2 compatible with OC4A (Leonardo Pin13)
// We configure the Timer4 for 11 bits PWM (enhacend precision) and 16.3ms period (OK for most servos)
// Resolution: 8us per step (this is OK for servos, around 175 steps for typical servo)

void initServo()
{
  int temp;
   
  servo1.attach(3);
  servo2.attach(4);
  servo1_ready=true;
  servo2_ready=true;
}

void disableServo1()
{
  servo1_ready=false;
  servo1.detach();
}

void disableServo2()
{
  servo2_ready=false;
  servo2.detach();
}

void enableServo1()
{
  servo1_ready=true;
  servo1.attach(3);
}

void enableServo2()
{
  servo2_ready=true;
  servo2.attach(4);
}


// move servo1
void moveServo1(int pwm)
{
  servo1.writeMicroseconds(pwm);
}

// move servo2 
void moveServo2(int pwm)
{
  servo2.writeMicroseconds(pwm);
}
