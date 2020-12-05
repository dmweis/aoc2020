var searchIndex = JSON.parse('{\
"aoc2020":{"doc":"","i":[[3,"Args","aoc2020","",null,null],[12,"day","","",0,null],[5,"main","","",null,[[]]],[0,"day_1","","",null,null],[4,"Day1Error","aoc2020::day_1","",null,null],[13,"FailedToFindPair","","",1,null],[5,"input","","",null,[[]]],[5,"task_1","","",null,[[],["result",6]]],[5,"task_2","","",null,[[],["result",6]]],[5,"run","","",null,[[]]],[0,"day_2","aoc2020","",null,null],[3,"PasswordAndPolicy","aoc2020::day_2","",null,null],[12,"first_number","","",2,null],[12,"second_number","","",2,null],[12,"letter","","",2,null],[12,"password","","",2,null],[4,"Day2Error","","",null,null],[13,"FailedToParsePolicy","","",3,null],[5,"input","","",null,[[]]],[5,"task_1","","",null,[[]]],[5,"task_2","","",null,[[]]],[5,"run","","",null,[[]]],[11,"parse","","",2,[[],["result",6]]],[11,"is_valid_first_policy","","",2,[[]]],[11,"is_valid_second_policy","","",2,[[]]],[0,"day_3","aoc2020","",null,null],[3,"TreeMaze","aoc2020::day_3","",null,null],[12,"trees","","",4,null],[12,"height","","",4,null],[12,"width","","",4,null],[3,"Slope","","",null,null],[12,"right","","",5,null],[12,"down","","",5,null],[4,"DayError","","",null,null],[13,"MapNotFormatted","","",6,null],[13,"PointOutOfBounds","","",6,null],[5,"input","","",null,[[]]],[5,"check_slope","","",null,[[["treemaze",3],["slope",3]],["result",6]]],[5,"task_1","","",null,[[],["result",6]]],[5,"task_2","","",null,[[],["result",6]]],[5,"run","","",null,[[]]],[17,"TREE","","",null,null],[17,"EMPTY","","",null,null],[11,"is_tree","","Check if location has a tree Will automatically handle…",4,[[],["result",6]]],[11,"new","","",5,[[]]],[0,"day_4","aoc2020","",null,null],[3,"Passport","aoc2020::day_4","",null,null],[12,"fields","","",7,null],[4,"Height","","",null,null],[13,"CM","","",8,null],[13,"IN","","",8,null],[5,"input","","",null,[[]]],[5,"validate_color","","",null,[[]]],[5,"validate_eye","","",null,[[]]],[5,"validate_passport_number","","",null,[[]]],[5,"parse_file","","",null,[[],[["vec",3],["passport",3]]]],[5,"task_1","","",null,[[]]],[5,"task_2","","",null,[[]]],[5,"run","","",null,[[]]],[17,"NEEDED_KEYS","","",null,null],[11,"is_valid_credential","","",7,[[]]],[11,"is_valid_strict","","",7,[[]]],[0,"day_5","aoc2020","",null,null],[3,"SeatLocation","aoc2020::day_5","",null,null],[12,"row","","",9,null],[12,"column","","",9,null],[12,"id","","",9,null],[4,"DayError","","",null,null],[13,"FailedParsingPosition","","",10,null],[13,"FailedToFindSeat","","",10,null],[5,"input","","",null,[[]]],[5,"calculate_row_rec","","",null,[[],["result",6]]],[5,"calculate_row","","",null,[[],["result",6]]],[5,"calculate_column_rec","","",null,[[],["result",6]]],[5,"calculate_column","","",null,[[],["result",6]]],[5,"task_1","","",null,[[],["result",6]]],[5,"task_2","","",null,[[],["result",6]]],[5,"run","","",null,[[]]],[17,"FRONT","","",null,null],[17,"BACK","","",null,null],[17,"LEFT","","",null,null],[17,"RIGHT","","",null,null],[11,"parse","","",9,[[],[["result",6],["seatlocation",3]]]],[11,"from","aoc2020","",0,[[]]],[11,"into","","",0,[[]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"from","aoc2020::day_1","",1,[[]]],[11,"into","","",1,[[]]],[11,"to_string","","",1,[[],["string",3]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"try_into","","",1,[[],["result",4]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"from","aoc2020::day_2","",2,[[]]],[11,"into","","",2,[[]]],[11,"borrow","","",2,[[]]],[11,"borrow_mut","","",2,[[]]],[11,"try_from","","",2,[[],["result",4]]],[11,"try_into","","",2,[[],["result",4]]],[11,"type_id","","",2,[[],["typeid",3]]],[11,"from","","",3,[[]]],[11,"into","","",3,[[]]],[11,"to_string","","",3,[[],["string",3]]],[11,"borrow","","",3,[[]]],[11,"borrow_mut","","",3,[[]]],[11,"try_from","","",3,[[],["result",4]]],[11,"try_into","","",3,[[],["result",4]]],[11,"type_id","","",3,[[],["typeid",3]]],[11,"from","aoc2020::day_3","",4,[[]]],[11,"into","","",4,[[]]],[11,"borrow","","",4,[[]]],[11,"borrow_mut","","",4,[[]]],[11,"try_from","","",4,[[],["result",4]]],[11,"try_into","","",4,[[],["result",4]]],[11,"type_id","","",4,[[],["typeid",3]]],[11,"from","","",5,[[]]],[11,"into","","",5,[[]]],[11,"borrow","","",5,[[]]],[11,"borrow_mut","","",5,[[]]],[11,"try_from","","",5,[[],["result",4]]],[11,"try_into","","",5,[[],["result",4]]],[11,"type_id","","",5,[[],["typeid",3]]],[11,"from","","",6,[[]]],[11,"into","","",6,[[]]],[11,"to_string","","",6,[[],["string",3]]],[11,"borrow","","",6,[[]]],[11,"borrow_mut","","",6,[[]]],[11,"try_from","","",6,[[],["result",4]]],[11,"try_into","","",6,[[],["result",4]]],[11,"type_id","","",6,[[],["typeid",3]]],[11,"from","aoc2020::day_4","",7,[[]]],[11,"into","","",7,[[]]],[11,"borrow","","",7,[[]]],[11,"borrow_mut","","",7,[[]]],[11,"try_from","","",7,[[],["result",4]]],[11,"try_into","","",7,[[],["result",4]]],[11,"type_id","","",7,[[],["typeid",3]]],[11,"from","","",8,[[]]],[11,"into","","",8,[[]]],[11,"borrow","","",8,[[]]],[11,"borrow_mut","","",8,[[]]],[11,"try_from","","",8,[[],["result",4]]],[11,"try_into","","",8,[[],["result",4]]],[11,"type_id","","",8,[[],["typeid",3]]],[11,"from","aoc2020::day_5","",9,[[]]],[11,"into","","",9,[[]]],[11,"borrow","","",9,[[]]],[11,"borrow_mut","","",9,[[]]],[11,"try_from","","",9,[[],["result",4]]],[11,"try_into","","",9,[[],["result",4]]],[11,"type_id","","",9,[[],["typeid",3]]],[11,"equivalent","","",9,[[]]],[11,"from","","",10,[[]]],[11,"into","","",10,[[]]],[11,"to_string","","",10,[[],["string",3]]],[11,"borrow","","",10,[[]]],[11,"borrow_mut","","",10,[[]]],[11,"try_from","","",10,[[],["result",4]]],[11,"try_into","","",10,[[],["result",4]]],[11,"type_id","","",10,[[],["typeid",3]]],[11,"from","aoc2020::day_4","",7,[[]]],[11,"eq","aoc2020::day_5","",9,[[["seatlocation",3]]]],[11,"ne","","",9,[[["seatlocation",3]]]],[11,"fmt","aoc2020::day_1","",1,[[["formatter",3]],["result",6]]],[11,"fmt","aoc2020::day_2","",3,[[["formatter",3]],["result",6]]],[11,"fmt","aoc2020::day_3","",6,[[["formatter",3]],["result",6]]],[11,"fmt","aoc2020::day_5","",10,[[["formatter",3]],["result",6]]],[11,"fmt","","",9,[[["formatter",3]],["result",6]]],[11,"fmt","aoc2020::day_1","",1,[[["formatter",3]],["result",6]]],[11,"fmt","aoc2020::day_2","",3,[[["formatter",3]],["result",6]]],[11,"fmt","aoc2020::day_3","",6,[[["formatter",3]],["result",6]]],[11,"fmt","aoc2020::day_5","",10,[[["formatter",3]],["result",6]]],[11,"from_str","aoc2020::day_2","",2,[[],["result",6]]],[11,"from_str","aoc2020::day_3","",4,[[],["result",6]]],[11,"into_app","aoc2020","",0,[[],["app",3]]],[11,"augment_clap","","",0,[[["app",3]],["app",3]]],[11,"from_arg_matches","","",0,[[["argmatches",3]]]]],"p":[[3,"Args"],[4,"Day1Error"],[3,"PasswordAndPolicy"],[4,"Day2Error"],[3,"TreeMaze"],[3,"Slope"],[4,"DayError"],[3,"Passport"],[4,"Height"],[3,"SeatLocation"],[4,"DayError"]]}\
}');
addSearchOptions(searchIndex);initSearch(searchIndex);