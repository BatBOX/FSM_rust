use crate::player::player_state_machine::PlayerStateMachine;
use crate::utils::character_state_common::{CharacterResource};
use godot::classes::{
    AnimatedSprite2D, AnimationPlayer, CharacterBody2D, ICharacterBody2D, Input, InputEvent,
};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Player {
    #[export]
    speed: f64,
    animated_sprite: Gd<AnimatedSprite2D>,
    pub(crate) animation_player: Gd<AnimationPlayer>,
    pub(crate) state_machine: Gd<PlayerStateMachine>,
    resource: Gd<CharacterResource>,
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        // 创建资源
        let resource = CharacterResource::new();

        Self {
            speed: 50.0,
            animated_sprite: AnimatedSprite2D::new_alloc(),
            animation_player: AnimationPlayer::new_alloc(),
            state_machine: PlayerStateMachine::new_alloc(),
            resource,
            base,
        }
    }

    fn process(&mut self, delta: f64) {
        // 处理非物理逻辑（动画、状态机）
        self.state_machine.bind_mut().handle_process(delta);
    }

    fn physics_process(&mut self, delta: f64) {
        // 物理相关逻辑
        self.state_machine.bind_mut().handle_physics_process(delta);

        // 执行移动
        self.base_mut().move_and_slide();

        // 保持人物朝向
        let direction = self.get_input_direction();
        if !direction.is_zero_approx() {
            self.resource
                .bind_mut()
                .set_last_facing_direction(direction);
            self.update_sprite_direction(direction);
        }
    }

    fn ready(&mut self) {
        // 获取节点引用
        self.animated_sprite = self.base().get_node_as("AnimatedSprite2D");
        self.animation_player = self.base().get_node_as("AnimationPlayer");

        // 获取状态机节点
        if self.base().has_node("PlayerStateMachine") {
            self.state_machine = self.base().get_node_as("PlayerStateMachine");

            // 设置资源
            self.resource
                .bind_mut()
                .set_animation_player(self.animation_player.clone());
            self.resource.bind_mut().set_speed(self.speed);

            // 更新状态机资源

            self.state_machine
                .bind_mut()
                .set_resource(self.resource.clone());
        } else {
            godot_warn!(
                "PlayerStateMachine节点未找到，请确保在Player节点下添加了PlayerStateMachine子节点"
            );
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        // 处理输入事件
        self.state_machine.bind_mut().handle_input(event);
    }
}

#[godot_api]
impl Player {
    pub fn get_input_direction(&self) -> Vector2 {
        let input = Input::singleton();
        Vector2::new(
            input.get_axis("move_left", "move_right"),
            input.get_axis("move_up", "move_down"),
        )
        .normalized_or_zero()
    }

    fn update_sprite_direction(&mut self, direction: Vector2) {
        // 更新精灵方向
        if direction.x != 0.0 {
            self.animated_sprite.set_flip_h(direction.x < 0.0);
        }
    }
}
