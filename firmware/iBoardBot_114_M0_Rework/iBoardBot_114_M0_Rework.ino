// Proxy commands from SerialUSB to Serial1 for communication with an ESP module

void setup() {
  // Initialize both Serial ports
  SerialUSB.begin(115200); // for communication with the computer
  Serial1.begin(115200);   // for communication with the ESP module
}

void loop() {
  // Check if data is available on SerialUSB
  if (SerialUSB.available()) {
    // Read the data from SerialUSB
    char data = SerialUSB.read();
    
    // Send the data to Serial1 (ESP module)
    Serial1.write(data);
  }

  // Check if data is available on Serial1 (ESP module)
  if (Serial1.available()) {
    // Read the data from Serial1 (ESP module)
    char data = Serial1.read();
    
    // Send the data to SerialUSB (computer)
    SerialUSB.write(data);
  }
}
