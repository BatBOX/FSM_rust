use crate::player::states_impl::run_state::{BackRunState, RunState, SideRunState};
use crate::utils::character_state_common::{
    CharacterResource, CharacterStateCommon, DirectionType, determine_direction_type,
    get_input_direction,
};
use crate::utils::state_machine::{GodotInitialState, GodotState, GodotStateTraits};
use godot::builtin::Vector2;
use godot::classes::{CharacterBody2D, INode};
use godot::obj::{Base, Gd};
use godot::prelude::{GodotClass, godot_api};

#[derive(GodotClass, Debug)]
#[class(base=Node)]
pub struct IdleState;

#[godot_api]
impl INode for IdleState {
    fn init(_base: Base<Self::Base>) -> Self {
        IdleState
    }
}

// 实现初始状态标记特性
impl GodotInitialState for IdleState {}

impl CharacterStateCommon for IdleState {
    fn get_animation_name(&self, animation_direction: &str) -> String {
        if animation_direction != "default" {
            format!("{}_idle", animation_direction)
        } else {
            "idle".to_string()
        }
    }
}

impl GodotState for IdleState {
    type Owner = CharacterBody2D;
    type Resource = CharacterResource;

    fn init(&self, owner: &mut Gd<CharacterBody2D>, resource: &mut Gd<CharacterResource>) {
        // 确保角色停止移动
        owner.set_velocity(Vector2::ZERO);

        // 播放对应的动画
        let animation_name = self.get_animation_name(resource.bind().get_animation_direction());
        resource.bind_mut().play_animation(&animation_name);
    }

    fn process(
        &self,
        _owner: &mut Gd<CharacterBody2D>,
        _resource: &mut Gd<CharacterResource>,
        _delta: f64,
    ) -> Option<Box<dyn GodotStateTraits<Owner = CharacterBody2D, Resource = CharacterResource>>>
    {
        let direction = get_input_direction();

        if !direction.is_zero_approx() {
            // 根据方向类型选择适当的状态
            let direction_type = determine_direction_type(direction);

            return match direction_type {
                DirectionType::Back => Some(Box::new(BackRunState)),
                DirectionType::Side => Some(Box::new(SideRunState)),
                DirectionType::Default => Some(Box::new(RunState)),
            };
        }

        None
    }
}

// 后向空闲状态
#[derive(Debug)]
pub struct BackIdleState;

impl CharacterStateCommon for BackIdleState {
    fn get_animation_name(&self, _animation_direction: &str) -> String {
        "back_idle".to_string()
    }
}

impl GodotInitialState for BackIdleState {}

impl GodotState for BackIdleState {
    type Owner = CharacterBody2D;
    type Resource = CharacterResource;

    fn init(&self, owner: &mut Gd<CharacterBody2D>, resource: &mut Gd<CharacterResource>) {
        // 确保角色停止移动
        owner.set_velocity(Vector2::ZERO);

        // 播放对应的动画
        let animation_name = self.get_animation_name(resource.bind().get_animation_direction());
        resource.bind_mut().play_animation(&animation_name);
    }

    fn process(
        &self,
        _owner: &mut Gd<CharacterBody2D>,
        _resource: &mut Gd<CharacterResource>,
        _delta: f64,
    ) -> Option<Box<dyn GodotStateTraits<Owner = CharacterBody2D, Resource = CharacterResource>>>
    {
        let direction = get_input_direction();

        if !direction.is_zero_approx() {
            // 根据方向类型选择适当的状态
            let direction_type = determine_direction_type(direction);

            return match direction_type {
                DirectionType::Back => Some(Box::new(BackRunState)),
                DirectionType::Side => Some(Box::new(SideRunState)),
                DirectionType::Default => Some(Box::new(RunState)),
            };
        }

        None
    }
}

// 侧向空闲状态
#[derive(Debug)]
pub struct SideIdleState;

impl CharacterStateCommon for SideIdleState {
    fn get_animation_name(&self, _animation_direction: &str) -> String {
        "side_idle".to_string()
    }
}

impl GodotInitialState for SideIdleState {}

impl GodotState for SideIdleState {
    type Owner = CharacterBody2D;
    type Resource = CharacterResource;

    fn init(&self, owner: &mut Gd<CharacterBody2D>, resource: &mut Gd<CharacterResource>) {
        // 确保角色停止移动
        owner.set_velocity(Vector2::ZERO);

        // 播放对应的动画
        let animation_name = self.get_animation_name(resource.bind().get_animation_direction());
        resource.bind_mut().play_animation(&animation_name);
    }

    fn process(
        &self,
        _owner: &mut Gd<CharacterBody2D>,
        _resource: &mut Gd<CharacterResource>,
        _delta: f64,
    ) -> Option<Box<dyn GodotStateTraits<Owner = CharacterBody2D, Resource = CharacterResource>>>
    {
        let direction = get_input_direction();

        if !direction.is_zero_approx() {
            // 根据方向类型选择适当的状态
            let direction_type = determine_direction_type(direction);

            return match direction_type {
                DirectionType::Back => Some(Box::new(BackRunState)),
                DirectionType::Side => Some(Box::new(SideRunState)),
                DirectionType::Default => Some(Box::new(RunState)),
            };
        }

        None
    }
}
