fn main() {

    let a = -3.711;
    let thrust = [a + 0.0, a + 1.0, a + 2.0, a + 3.0, a + 4.0];
    let mut second = 0;
    let mut actual_speed = 0 as f64;
    let mut actual_y = 2500.0;
    let mut max_thrust = false;
    let ground_height = 100.0;

   while actual_y > ground_height {

       let delta_y = actual_speed + a/2.0;
       actual_speed = -1.0 * f64::sqrt(f64::abs(actual_speed * actual_speed + 2.0 * a * delta_y));
       actual_y += delta_y;

        let final_thrust =
            if max_thrust {
                4
            } else {
                let (next_pos, next_vel) = calculate_pos_vel(
                    actual_y,
                    actual_speed,
                    thrust[1],
                );

                let (next_pos, next_vel) = calculate_pos_vel(
                    next_pos,
                    next_vel,
                    thrust[2],
                );

                let (next_pos, next_vel) = calculate_pos_vel(
                    next_pos,
                    next_vel,
                    thrust[3],
                );

                let delta_y_to_ground = ground_height - next_pos;
                let max_thrust_velocity_next_turn = f64::sqrt((next_vel * next_vel) + 2.0 * thrust[4] * delta_y_to_ground);

                if max_thrust_velocity_next_turn.ceil() as i64 >= 40 {
                    max_thrust = true;
                    4
                } else {
                    0
                }
            };

        println!("0 {final_thrust}");
        second += 1
    }

    println!("second {second}");
}

fn calculate_pos_vel(
    initial_pos: f64,
    initial_vel: f64,
    acceleration: f64,
) -> (f64, f64) {
    let final_pos = initial_pos + initial_vel + acceleration / 2.0;
    let final_vel = initial_vel + acceleration;

    (final_pos, final_vel)
}
