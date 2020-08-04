
// map with lazy failure
let extracted = videos
    .map(|x|json_to_video(x))
    .take_while(|x|x.is_ok())
    .map(|x|x.ok().unwrap())
    .collect() 
