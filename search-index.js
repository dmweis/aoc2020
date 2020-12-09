var searchIndex = JSON.parse('{\
"aoc2020":{"doc":"","i":[[3,"Args","aoc2020","",null,null],[12,"day","","",0,null],[5,"main","","",null,[[]]],[0,"day_1","","",null,null],[4,"Day1Error","aoc2020::day_1","",null,null],[13,"FailedToFindPair","","",1,null],[5,"input","","",null,[[]]],[5,"task_1","","",null,[[],["result",6]]],[5,"task_2","","",null,[[],["result",6]]],[5,"run","","",null,[[]]],[0,"day_2","aoc2020","",null,null],[3,"PasswordAndPolicy","aoc2020::day_2","",null,null],[12,"first_number","","",2,null],[12,"second_number","","",2,null],[12,"letter","","",2,null],[12,"password","","",2,null],[4,"Day2Error","","",null,null],[13,"FailedToParsePolicy","","",3,null],[5,"input","","",null,[[]]],[5,"task_1","","",null,[[]]],[5,"task_2","","",null,[[]]],[5,"run","","",null,[[]]],[11,"parse","","",2,[[],["result",6]]],[11,"is_valid_first_policy","","",2,[[]]],[11,"is_valid_second_policy","","",2,[[]]],[0,"day_3","aoc2020","",null,null],[3,"TreeMaze","aoc2020::day_3","",null,null],[12,"trees","","",4,null],[12,"height","","",4,null],[12,"width","","",4,null],[3,"Slope","","",null,null],[12,"right","","",5,null],[12,"down","","",5,null],[4,"DayError","","",null,null],[13,"MapNotFormatted","","",6,null],[13,"PointOutOfBounds","","",6,null],[5,"input","","",null,[[]]],[5,"check_slope","","",null,[[["treemaze",3],["slope",3]],["result",6]]],[5,"task_1","","",null,[[],["result",6]]],[5,"task_2","","",null,[[],["result",6]]],[5,"run","","",null,[[]]],[17,"TREE","","",null,null],[17,"EMPTY","","",null,null],[11,"is_tree","","Check if location has a tree Will automatically handle…",4,[[],["result",6]]],[11,"new","","",5,[[]]],[0,"day_4","aoc2020","",null,null],[3,"Passport","aoc2020::day_4","",null,null],[12,"fields","","",7,null],[4,"Height","","",null,null],[13,"CM","","",8,null],[13,"IN","","",8,null],[5,"input","","",null,[[]]],[5,"validate_color","","",null,[[]]],[5,"validate_eye","","",null,[[]]],[5,"validate_passport_number","","",null,[[]]],[5,"parse_file","","",null,[[],[["vec",3],["passport",3]]]],[5,"task_1","","",null,[[]]],[5,"task_2","","",null,[[]]],[5,"run","","",null,[[]]],[17,"NEEDED_KEYS","","",null,null],[11,"is_valid_credential","","",7,[[]]],[11,"is_valid_strict","","",7,[[]]],[0,"day_5","aoc2020","",null,null],[3,"SeatLocation","aoc2020::day_5","",null,null],[12,"row","","",9,null],[12,"column","","",9,null],[12,"id","","",9,null],[4,"DayError","","",null,null],[13,"FailedParsingPosition","","",10,null],[13,"FailedToFindSeat","","",10,null],[5,"input","","",null,[[]]],[5,"calculate_row_rec","","",null,[[],["result",6]]],[5,"calculate_row","","",null,[[],["result",6]]],[5,"calculate_column_rec","","",null,[[],["result",6]]],[5,"calculate_column","","",null,[[],["result",6]]],[5,"task_1","","",null,[[],["result",6]]],[5,"task_2","","",null,[[],["result",6]]],[5,"run","","",null,[[]]],[17,"FRONT","","",null,null],[17,"BACK","","",null,null],[17,"LEFT","","",null,null],[17,"RIGHT","","",null,null],[11,"parse","","",9,[[],[["seatlocation",3],["result",6]]]],[0,"day_6","aoc2020","",null,null],[5,"input","aoc2020::day_6","",null,[[]]],[5,"task_1","","",null,[[]]],[5,"task_2","","",null,[[]]],[5,"run","","",null,[[]]],[0,"day_7","aoc2020","",null,null],[3,"RULE_REGEX","aoc2020::day_7","",null,null],[12,"__private_field","","",11,null],[3,"ContainedBag","","",null,null],[12,"color","","",12,null],[12,"count","","",12,null],[3,"BagRule","","",null,null],[12,"parent","","",13,null],[12,"children","","",13,null],[3,"RuleSet","","",null,null],[12,"bag_to_containers","","",14,null],[12,"bag_to_children","","",14,null],[4,"DayError","","",null,null],[13,"FailedParsingRule","","",15,null],[13,"FailedTreeTraversal","","",15,null],[5,"input","","",null,[[]]],[5,"task_1","","",null,[[],["result",6]]],[5,"task_2","","",null,[[],["result",6]]],[5,"run","","",null,[[]]],[17,"MY_BAG","","",null,null],[11,"parse","","",13,[[],[["bagrule",3],["result",6]]]],[11,"parse_ruleset","","",14,[[],[["result",6],["ruleset",3]]]],[11,"count_topmost_containers","","",14,[[]]],[11,"count_contained","","",14,[[],["result",6]]],[0,"day_8","aoc2020","",null,null],[3,"Computer","aoc2020::day_8","",null,null],[12,"commands","","",16,null],[3,"ComputerMutator","","",null,null],[12,"original","","",17,null],[12,"next_mutate","","",17,null],[4,"DayError","","",null,null],[13,"ParsingError","","",18,null],[13,"UnknownInstruction","","",18,null],[13,"MissingInstruction","","",18,null],[13,"FailedToMutate","","",18,null],[13,"InfiniteLoop","","",18,null],[4,"Command","","",null,null],[13,"Acc","","",19,null],[13,"Nop","","",19,null],[13,"Jmp","","",19,null],[5,"input","","",null,[[]]],[5,"task_1","","",null,[[],["result",6]]],[5,"task_2","","",null,[[],["result",6]]],[5,"run","","",null,[[]]],[11,"parse","","",19,[[],[["result",6],["command",4]]]],[11,"parse","","",16,[[],[["result",6],["computer",3]]]],[11,"run_blocking_no_repeat","","",16,[[],["result",6]]],[11,"run_blocking_correct_termination","","",16,[[],["result",6]]],[11,"new","","",17,[[["computer",3]]]],[11,"verify_next","","",17,[[],[["option",4],["result",6]]]],[11,"from","aoc2020","",0,[[]]],[11,"into","","",0,[[]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"from","aoc2020::day_1","",1,[[]]],[11,"into","","",1,[[]]],[11,"to_string","","",1,[[],["string",3]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"try_into","","",1,[[],["result",4]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"from","aoc2020::day_2","",2,[[]]],[11,"into","","",2,[[]]],[11,"borrow","","",2,[[]]],[11,"borrow_mut","","",2,[[]]],[11,"try_from","","",2,[[],["result",4]]],[11,"try_into","","",2,[[],["result",4]]],[11,"type_id","","",2,[[],["typeid",3]]],[11,"from","","",3,[[]]],[11,"into","","",3,[[]]],[11,"to_string","","",3,[[],["string",3]]],[11,"borrow","","",3,[[]]],[11,"borrow_mut","","",3,[[]]],[11,"try_from","","",3,[[],["result",4]]],[11,"try_into","","",3,[[],["result",4]]],[11,"type_id","","",3,[[],["typeid",3]]],[11,"from","aoc2020::day_3","",4,[[]]],[11,"into","","",4,[[]]],[11,"borrow","","",4,[[]]],[11,"borrow_mut","","",4,[[]]],[11,"try_from","","",4,[[],["result",4]]],[11,"try_into","","",4,[[],["result",4]]],[11,"type_id","","",4,[[],["typeid",3]]],[11,"from","","",5,[[]]],[11,"into","","",5,[[]]],[11,"borrow","","",5,[[]]],[11,"borrow_mut","","",5,[[]]],[11,"try_from","","",5,[[],["result",4]]],[11,"try_into","","",5,[[],["result",4]]],[11,"type_id","","",5,[[],["typeid",3]]],[11,"from","","",6,[[]]],[11,"into","","",6,[[]]],[11,"to_string","","",6,[[],["string",3]]],[11,"borrow","","",6,[[]]],[11,"borrow_mut","","",6,[[]]],[11,"try_from","","",6,[[],["result",4]]],[11,"try_into","","",6,[[],["result",4]]],[11,"type_id","","",6,[[],["typeid",3]]],[11,"from","aoc2020::day_4","",7,[[]]],[11,"into","","",7,[[]]],[11,"borrow","","",7,[[]]],[11,"borrow_mut","","",7,[[]]],[11,"try_from","","",7,[[],["result",4]]],[11,"try_into","","",7,[[],["result",4]]],[11,"type_id","","",7,[[],["typeid",3]]],[11,"from","","",8,[[]]],[11,"into","","",8,[[]]],[11,"borrow","","",8,[[]]],[11,"borrow_mut","","",8,[[]]],[11,"try_from","","",8,[[],["result",4]]],[11,"try_into","","",8,[[],["result",4]]],[11,"type_id","","",8,[[],["typeid",3]]],[11,"from","aoc2020::day_5","",9,[[]]],[11,"into","","",9,[[]]],[11,"borrow","","",9,[[]]],[11,"borrow_mut","","",9,[[]]],[11,"try_from","","",9,[[],["result",4]]],[11,"try_into","","",9,[[],["result",4]]],[11,"type_id","","",9,[[],["typeid",3]]],[11,"equivalent","","",9,[[]]],[11,"from","","",10,[[]]],[11,"into","","",10,[[]]],[11,"to_string","","",10,[[],["string",3]]],[11,"borrow","","",10,[[]]],[11,"borrow_mut","","",10,[[]]],[11,"try_from","","",10,[[],["result",4]]],[11,"try_into","","",10,[[],["result",4]]],[11,"type_id","","",10,[[],["typeid",3]]],[11,"from","aoc2020::day_7","",11,[[]]],[11,"into","","",11,[[]]],[11,"borrow","","",11,[[]]],[11,"borrow_mut","","",11,[[]]],[11,"try_from","","",11,[[],["result",4]]],[11,"try_into","","",11,[[],["result",4]]],[11,"type_id","","",11,[[],["typeid",3]]],[11,"from","","",12,[[]]],[11,"into","","",12,[[]]],[11,"to_owned","","",12,[[]]],[11,"clone_into","","",12,[[]]],[11,"borrow","","",12,[[]]],[11,"borrow_mut","","",12,[[]]],[11,"try_from","","",12,[[],["result",4]]],[11,"try_into","","",12,[[],["result",4]]],[11,"type_id","","",12,[[],["typeid",3]]],[11,"equivalent","","",12,[[]]],[11,"from","","",13,[[]]],[11,"into","","",13,[[]]],[11,"borrow","","",13,[[]]],[11,"borrow_mut","","",13,[[]]],[11,"try_from","","",13,[[],["result",4]]],[11,"try_into","","",13,[[],["result",4]]],[11,"type_id","","",13,[[],["typeid",3]]],[11,"equivalent","","",13,[[]]],[11,"from","","",14,[[]]],[11,"into","","",14,[[]]],[11,"borrow","","",14,[[]]],[11,"borrow_mut","","",14,[[]]],[11,"try_from","","",14,[[],["result",4]]],[11,"try_into","","",14,[[],["result",4]]],[11,"type_id","","",14,[[],["typeid",3]]],[11,"equivalent","","",14,[[]]],[11,"from","","",15,[[]]],[11,"into","","",15,[[]]],[11,"to_string","","",15,[[],["string",3]]],[11,"borrow","","",15,[[]]],[11,"borrow_mut","","",15,[[]]],[11,"try_from","","",15,[[],["result",4]]],[11,"try_into","","",15,[[],["result",4]]],[11,"type_id","","",15,[[],["typeid",3]]],[11,"from","aoc2020::day_8","",16,[[]]],[11,"into","","",16,[[]]],[11,"to_owned","","",16,[[]]],[11,"clone_into","","",16,[[]]],[11,"borrow","","",16,[[]]],[11,"borrow_mut","","",16,[[]]],[11,"try_from","","",16,[[],["result",4]]],[11,"try_into","","",16,[[],["result",4]]],[11,"type_id","","",16,[[],["typeid",3]]],[11,"equivalent","","",16,[[]]],[11,"from","","",17,[[]]],[11,"into","","",17,[[]]],[11,"borrow","","",17,[[]]],[11,"borrow_mut","","",17,[[]]],[11,"try_from","","",17,[[],["result",4]]],[11,"try_into","","",17,[[],["result",4]]],[11,"type_id","","",17,[[],["typeid",3]]],[11,"from","","",18,[[]]],[11,"into","","",18,[[]]],[11,"to_string","","",18,[[],["string",3]]],[11,"borrow","","",18,[[]]],[11,"borrow_mut","","",18,[[]]],[11,"try_from","","",18,[[],["result",4]]],[11,"try_into","","",18,[[],["result",4]]],[11,"type_id","","",18,[[],["typeid",3]]],[11,"from","","",19,[[]]],[11,"into","","",19,[[]]],[11,"to_owned","","",19,[[]]],[11,"clone_into","","",19,[[]]],[11,"borrow","","",19,[[]]],[11,"borrow_mut","","",19,[[]]],[11,"try_from","","",19,[[],["result",4]]],[11,"try_into","","",19,[[],["result",4]]],[11,"type_id","","",19,[[],["typeid",3]]],[11,"equivalent","","",19,[[]]],[11,"from","aoc2020::day_4","",7,[[]]],[11,"clone","aoc2020::day_7","",12,[[],["containedbag",3]]],[11,"clone","aoc2020::day_8","",19,[[],["command",4]]],[11,"clone","","",16,[[],["computer",3]]],[11,"eq","aoc2020::day_5","",9,[[["seatlocation",3]]]],[11,"ne","","",9,[[["seatlocation",3]]]],[11,"eq","aoc2020::day_7","",12,[[["containedbag",3]]]],[11,"ne","","",12,[[["containedbag",3]]]],[11,"eq","","",13,[[["bagrule",3]]]],[11,"ne","","",13,[[["bagrule",3]]]],[11,"eq","","",14,[[["ruleset",3]]]],[11,"ne","","",14,[[["ruleset",3]]]],[11,"eq","aoc2020::day_8","",19,[[["command",4]]]],[11,"ne","","",19,[[["command",4]]]],[11,"eq","","",16,[[["computer",3]]]],[11,"ne","","",16,[[["computer",3]]]],[11,"deref","aoc2020::day_7","",11,[[],["regex",3]]],[11,"fmt","aoc2020::day_1","",1,[[["formatter",3]],["result",6]]],[11,"fmt","aoc2020::day_2","",3,[[["formatter",3]],["result",6]]],[11,"fmt","aoc2020::day_3","",6,[[["formatter",3]],["result",6]]],[11,"fmt","aoc2020::day_5","",10,[[["formatter",3]],["result",6]]],[11,"fmt","","",9,[[["formatter",3]],["result",6]]],[11,"fmt","aoc2020::day_7","",15,[[["formatter",3]],["result",6]]],[11,"fmt","","",12,[[["formatter",3]],["result",6]]],[11,"fmt","","",13,[[["formatter",3]],["result",6]]],[11,"fmt","","",14,[[["formatter",3]],["result",6]]],[11,"fmt","aoc2020::day_8","",18,[[["formatter",3]],["result",6]]],[11,"fmt","","",19,[[["formatter",3]],["result",6]]],[11,"fmt","","",16,[[["formatter",3]],["result",6]]],[11,"fmt","aoc2020::day_1","",1,[[["formatter",3]],["result",6]]],[11,"fmt","aoc2020::day_2","",3,[[["formatter",3]],["result",6]]],[11,"fmt","aoc2020::day_3","",6,[[["formatter",3]],["result",6]]],[11,"fmt","aoc2020::day_5","",10,[[["formatter",3]],["result",6]]],[11,"fmt","aoc2020::day_7","",15,[[["formatter",3]],["result",6]]],[11,"fmt","aoc2020::day_8","",18,[[["formatter",3]],["result",6]]],[11,"from_str","aoc2020::day_2","",2,[[],["result",6]]],[11,"from_str","aoc2020::day_3","",4,[[],["result",6]]],[11,"initialize","aoc2020::day_7","",11,[[]]],[11,"into_app","aoc2020","",0,[[],["app",3]]],[11,"augment_clap","","",0,[[["app",3]],["app",3]]],[11,"from_arg_matches","","",0,[[["argmatches",3]]]]],"p":[[3,"Args"],[4,"Day1Error"],[3,"PasswordAndPolicy"],[4,"Day2Error"],[3,"TreeMaze"],[3,"Slope"],[4,"DayError"],[3,"Passport"],[4,"Height"],[3,"SeatLocation"],[4,"DayError"],[3,"RULE_REGEX"],[3,"ContainedBag"],[3,"BagRule"],[3,"RuleSet"],[4,"DayError"],[3,"Computer"],[3,"ComputerMutator"],[4,"DayError"],[4,"Command"]]}\
}');
addSearchOptions(searchIndex);initSearch(searchIndex);