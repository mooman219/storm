
use storm::input::message::*;
use storm::render::message::*;
use storm::utility::slotmap::*;
use storm::cgmath::Vector2;
use storm::render::color;

//////////
/// Main Driver for conways game of life
///////

const MAP_X_SIZE: usize = 50;

pub enum CurrentActiveFrame {
    A,
    B
}

pub struct System {
    a_frame: [[bool; MAP_X_SIZE]; MAP_X_SIZE],
    b_frame: [[bool; MAP_X_SIZE]; MAP_X_SIZE],
    index_tokens: Vec<Vec<IndexToken>>,
    current_active_frame: CurrentActiveFrame,
    extra_slow: i32
}

impl System {
    pub fn new(render: &mut RenderProducer)-> System {
        
        //gotten through person experimentation
        render.set_scale(0.001f32);
        //precreate our index tokens for the board, they will never change in number
        let mut index_tokens: Vec<Vec<IndexToken>> = vec![];
        for x in 0..MAP_X_SIZE {
            index_tokens.push(vec![]);
            for y in 0..MAP_X_SIZE { 
                index_tokens[x].push(render.create_rect(Vector2::new(x as f32 * 10.0, y as f32 * 10.0), Vector2::new(x as f32 * 10.0, y as f32 * 10.0), color::PURPLE));
            }
        }
        render.send();
        
        System {
            a_frame: [[false; MAP_X_SIZE]; MAP_X_SIZE],
            b_frame: [[false; MAP_X_SIZE]; MAP_X_SIZE],
            current_active_frame: CurrentActiveFrame::A,
            index_tokens,
            extra_slow: 0
        }
    }

    pub fn create_blinker(&mut self, x: usize, y: usize) {
        if x > 0 && x < 49 && y > 0 && y < 50{
            let use_frame;
            match self.current_active_frame {
                CurrentActiveFrame::A => {
                    use_frame = &mut self.a_frame;
                },
                CurrentActiveFrame::B => {
                    use_frame = &mut self.b_frame;
                }
            }
            use_frame[x][y] = true;
            use_frame[x - 1][y] = true;
            use_frame[x + 1][y] = true;
            use_frame[x][y + 1] = true;
            use_frame[x][y - 1] = true;
        }
    }

    //helper functions, will count the neighbors for any given x, y, in active_frame
    #[inline]
    pub fn neighbor_count(x: usize, y:usize, active_frame: &mut [[bool;MAP_X_SIZE];MAP_X_SIZE] ) -> usize {
        let mut count = 0;
        //needs to be between 0 and 3 to handle inclusive lower value, but exclusive higer value
        //and the fact that rust does not allow for negative iterations
        for loop_x in 0..3 {
            //create and check shifted x value
            let count_x = (x + loop_x) as i8 - 1;
            if count_x < 0 || count_x > (MAP_X_SIZE as i8) - 1 {
                continue;
            }

            let count_x = count_x as usize;

            for loop_y in 0..3 {
                
                //creted and check shifted y value
                let count_y = (y + loop_y) as i8 - 1;
                if count_y < 0 || count_y > (MAP_X_SIZE as i8) - 1 {
                    continue;
                }
                
                let count_y = count_y as usize;
                
                if count_x == x && count_y == y {
                    continue;
                }

                if active_frame[count_x][count_y] == true {
                    count+=1;
                }
            }
        }

        count
    }

    pub fn tick(&mut self, render: &mut RenderProducer) {
        self.extra_slow += 1;

        if self.extra_slow < 50 {
            return;
        }
        else {
            self.extra_slow = 0;
        }
        //check for current active frame, a_frame or b_frame, and set our use and write frames
        let use_frame;
        let write_frame;
        match self.current_active_frame {
            CurrentActiveFrame::A => {
                use_frame = &mut self.a_frame;
                write_frame = &mut self.b_frame;
            },
            CurrentActiveFrame::B => {
                use_frame = &mut self.b_frame;
                write_frame = &mut self.a_frame;
            }
        }

        for x in 0..MAP_X_SIZE {
            for y in 0..MAP_X_SIZE {
                let count = System::neighbor_count(x, y, use_frame);
                write_frame[x][y] = use_frame[x][y];
                //conways GOL 4 rules, for line cell
                if use_frame[x][y] == true {
                    //with fewer then 2 alive neighbors dies
                    if count < 2 {
                        write_frame[x][y] = false;
                    }
                    //with 2 or three neighbors surives
                    else if count == 2 || count == 3 {
                        write_frame[x][y] = true;
                    } 
                    //with greatern then 3 dies
                    else if count > 3 {
                        write_frame[x][y] = false;
                    }
                }
                //for any dead cell 
                else {
                    //with exactaly 3 alive neighbors comes alive
                    if count == 3 {
                        write_frame[x][y] = true;
                    }
                    
                }

                //if a cell is alive, it is orange, dead it is purple
                let use_color;
                if write_frame[x][y] {
                    use_color = color::ORANGE;
                }
                else {
                    use_color = color::PURPLE;
                }

                //update our cell to reflect its current status
                render.update_rect(&mut self.index_tokens[x][y], Vector2::new(x as f32 * 10.0, y as f32 * 10.0), Vector2::new(x as f32 * 10.0, y as f32 * 10.0), use_color);
            }
        }

        match self.current_active_frame {
            CurrentActiveFrame::A => {
                self.current_active_frame = CurrentActiveFrame::B;
            }
            CurrentActiveFrame::B => {
                self.current_active_frame = CurrentActiveFrame::A;
            }
        }
    }

    pub fn handle_input(&mut self, _input_frame: InputFrame) {

    }
}