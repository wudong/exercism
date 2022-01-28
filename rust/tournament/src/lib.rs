use std::collections::HashMap;
use std::fmt::Write as FmtWrite;

pub fn tally(match_results: &str) -> String {
    let mut result  = HashMap::<String,TeamResult>::new(); 
    let mut res = String::new();
    match_results.lines().for_each(
        |line| {
            let mut splits = line.split(";");
            let team1 = splits.next().unwrap();
            let team2 = splits.next().unwrap();
            let res = splits.next().unwrap();
            update_map(&mut result, (team1.to_string(), team2.to_string(), res.to_string()));
        }
    );

    let mut vv : Vec<(String, TeamResult)> = result.into_iter().collect();
    vv.sort_by(|x,y|{
        let comp = u32::cmp(&x.1.point, &y.1.point).reverse();
        match comp {
            std::cmp::Ordering::Equal=> {
                return String::cmp(&x.1.team_name, &y.1.team_name);
            },
            x => x,
        }
    });

    let _= write!(res, "{:<31}|{:^4}| {:^3}| {:^3}| {:^3}|{:>3}" , "Team", "MP", "W", "D", "L", "P");
    vv.iter().for_each(|f|{
        let _ = write!(res, "\n");
        let _= write!(res, "{:<31}| {:^3}| {:^3}| {:^3}| {:^3}|{:>3}", f.0, f.1.play, f.1.win, f.1.draw, f.1.lost, f.1.point);
    });

    res

}

fn update_map(res: &mut HashMap<String,TeamResult>, line_res: (String, String, String)) {
    match res.get_mut(&line_res.0){
        Some(result)=> {
            result.update(&line_res.2, true);
        },
        None => {
            let mut r = TeamResult::new(line_res.0.clone());
            r.update(&line_res.2, true);
            res.insert(line_res.0, r);
        },
    };

    match res.get_mut(&line_res.1){
        Some(result)=> {
            result.update(&line_res.2, false);
        },
        None => {
            let mut r = TeamResult::new(line_res.1.clone());
            r.update(&line_res.2, false);
            res.insert(line_res.1, r);
        },
    }
}

struct TeamResult {
    team_name: String,
    win: u32,
    draw: u32,
    lost: u32,
    play: u32,
    point: u32,
}

impl TeamResult {
    fn new(name: String) -> Self {
        Self {
            team_name: name,
            win: 0,
            draw:0,
            lost:0,
            play:0,
            point:0,
        }
    }

    fn win(&mut self) {
       self.play+=1;
       self.win+=1;
       self.point+=3;    
    }

    fn draw(&mut self) {
        self.play+=1;
        self.draw+=1;
        self.point+=1;    
     }

     fn lost(&mut self) {
        self.play+=1;
        self.lost+=1;
        self.point+=0;    
     }
 
    fn update(&mut self, res: &str, is_first: bool)     {
        match res {
            "win"=> {
                if is_first {
                    self.win();
                }else {
                    self.lost();
                };
            },
            "draw"=> {
                self.draw();
            },
            "loss"=> {
                if is_first {
                    self.lost();
                }else {
                    self.win();
                }
            }                        
            _=> ()
        }
    }
}