
// Движение интерфейса вместе с мышью
pub trait MouseMovementUI{
    fn interface_movement(&mut self,movement:[f64;2]);

    fn mouse_movement_ui(&mut self,event:&Event){
        if let Some(mut movement)=event.mouse_relative_args(){
            mouse_cursor.shift(movement);
            
            movement=self.get_mouse_movement();
            movement[0]=-movement[0];
            movement[1]=-movement[1];
            self.interface_movement(movement);
        }
    }
}

fn mouse_movement_ui(&mut self,event:&Event){
    if let Some(mut movement)=event.mouse_relative_args(){
        mouse_cursor.shift(movement);
        
        movement=self.get_mouse_movement();
        movement[0]=-movement[0];
        movement[1]=-movement[1];
        self.interface_movement(movement);
    }
}