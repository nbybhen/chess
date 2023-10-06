pub fn init(){
    let win = video_subsystem.window("CHESS", 800, 800)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
}