
// map with lazy failure
// collect() will throw errors inside the iterator
let extracted = videos
    .into_iter()
    .flat_map(|x| json_to_video(x))
    .collect();
