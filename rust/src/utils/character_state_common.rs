use godot::classes::AnimationPlayer;
use godot::prelude::*;

// 获取输入方向
pub fn get_input_direction() -> Vector2 {
    let input = Input::singleton();
    Vector2::new(
        input.get_axis("move_left", "move_right"),
        input.get_axis("move_up", "move_down"),
    )
    .normalized_or_zero()
}

// 根据输入方向确定应该使用哪种状态
pub fn determine_direction_type(direction: Vector2) -> DirectionType {
    if direction.is_zero_approx() {
        return DirectionType::Default;
    }
    let direction = direction.normalized_or_zero();

    match direction {
        dir if dir.y > 0.5 => DirectionType::Default,
        dir if dir.y < -0.5 => DirectionType::Back,
        dir if dir.x.abs() > 0.5 => DirectionType::Side,
        _ => DirectionType::Default,
    }
}

// 资源结构体，用于存储状态机需要的数据
#[derive(GodotClass)]
#[class(init, base=RefCounted)]
pub struct CharacterResource {
    animation_player: Option<Gd<AnimationPlayer>>,
    animation_direction: String,
    last_facing_direction: Vector2,
    speed: f64,
    base: Base<RefCounted>,
}

#[godot_api]
impl CharacterResource {
    pub fn new() -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            animation_player: None,
            animation_direction: "default".to_string(),
            last_facing_direction: Vector2::new(0.0, 0.0),
            speed: 50.0,
            base,
        })
    }

    pub fn set_animation_player(&mut self, animation_player: Gd<AnimationPlayer>) {
        self.animation_player = Some(animation_player);
    }

    pub fn get_animation_player(&self) -> Option<Gd<AnimationPlayer>> {
        self.animation_player.clone()
    }

    pub fn set_animation_direction(&mut self, direction: &str) {
        self.animation_direction = direction.to_string();
    }

    pub fn get_animation_direction(&self) -> &str {
        &self.animation_direction
    }

    pub fn set_last_facing_direction(&mut self, direction: Vector2) {
        self.last_facing_direction = direction;
    }

    pub fn get_last_facing_direction(&self) -> Vector2 {
        self.last_facing_direction
    }

    pub fn set_speed(&mut self, speed: f64) {
        self.speed = speed;
    }

    pub fn get_speed(&self) -> f64 {
        self.speed
    }

    // 播放动画
    pub fn play_animation(&mut self, animation_name: &str) {
        if let Some(mut animation_player) = self.get_animation_player() {
            animation_player.play_ex().name(animation_name).done();
        } else {
            godot_warn!("尝试播放动画 {}, 但AnimationPlayer未设置", animation_name);
        }
    }
}

// 基础状态特性
pub trait CharacterStateCommon {
    fn get_animation_name(&self, animation_direction: &str) -> String;

    // 获取状态的方向类型，用于状态转换逻辑
    fn get_direction_type(&self) -> DirectionType {
        DirectionType::Default
    }
}

// 方向类型枚举
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DirectionType {
    Default,
    Back,
    Side,
}
