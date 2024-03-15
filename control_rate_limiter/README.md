# Oneiros Rate Limiter

## Purpose
The purpose of the rate limiter is to have acceleration curves on the motor.  
The motor acceleration is too important for the performance of the robot. The inertia is dragging on, resulting in a poor adherence on the floor.  

## Usage
Simply input the motors values from the control package. You can choose the get the value out of the function or just borrow your values.  
You can customize the acceleration rat (ACCEL_RATE) and the deceleration rate (DECEL_RATE).  

These rates are depending sampling time. Meaning the result might differ depending on fast the function is called. Take this into account when adjusting your rate.
