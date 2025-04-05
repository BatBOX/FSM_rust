use crate::player::states_impl::idle_state::{BackIdleState, IdleState, SideIdleState};
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
pub struct RunState;

#[godot_api]
impl INode for RunState {
    fn init(_base: Base<Self::Base>) -> Self {
        RunState
    }
}

// 实现初始状态标记特性
impl GodotInitialState for RunState {}

impl CharacterStateCommon for RunState {
    fn get_animation_name(&self, animation_direction: &str) -> String {
        if animation_direction != "default" {
            format!("{}_running", animation_direction)
        } else {
            "running".to_string()
        }
    }

    fn get_direction_type(&self) -> DirectionType {
        DirectionType::Default
    }
}

impl GodotState for RunState {
    type Owner = CharacterBody2D;
    type Resource = CharacterResource;

    fn init(&self, _owner: &mut Gd<CharacterBody2D>, resource: &mut Gd<CharacterResource>) {
        // 修改：确保在进入RunState时设置为default朝向
        resource.bind_mut().set_animation_direction("default");
        let animation_name = self.get_animation_name("default");
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

        if direction.is_zero_approx() {
            Some(Box::new(IdleState))
        } else {
            // 检查方向是否改变，需要切换到其他运动状态
            let direction_type = determine_direction_type(direction);

            match direction_type {
                DirectionType::Back => Some(Box::new(BackRunState)),
                DirectionType::Side => Some(Box::new(SideRunState)),
                DirectionType::Default => None, // 保持当前状态
            }
        }
    }

    fn physics_process(
        &self,
        owner: &mut Gd<CharacterBody2D>,
        resource: &mut Gd<CharacterResource>,
        _delta: f64,
    ) -> Option<Box<dyn GodotStateTraits<Owner = CharacterBody2D, Resource = CharacterResource>>>
    {
        let direction = get_input_direction();

        if direction.is_zero_approx() {
            // 如果没有输入，立即停止移动
            owner.set_velocity(Vector2::ZERO);
            return Some(Box::new(IdleState));
        }

        // 更新速度
        let speed = resource.bind().get_speed() * 1.5; // 跑步时速度提升
        let velocity = direction * speed as f32;
        owner.set_velocity(velocity);

        None
    }
}

// 后向奔跑状态
#[derive(Debug)]
pub struct BackRunState;

impl CharacterStateCommon for BackRunState {
    fn get_animation_name(&self, _animation_direction: &str) -> String {
        "back_running".to_string()
    }
}

impl GodotInitialState for BackRunState {}

impl GodotState for BackRunState {
    type Owner = CharacterBody2D;
    type Resource = CharacterResource;

    fn init(&self, _owner: &mut Gd<CharacterBody2D>, resource: &mut Gd<CharacterResource>) {
        // 确保设置为back朝向
        resource.bind_mut().set_animation_direction("back");
        let animation_name = self.get_animation_name("back");
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

        if direction.is_zero_approx() {
            Some(Box::new(BackIdleState))
        } else {
            // 检查方向是否改变，需要切换到其他运动状态
            let direction_type = determine_direction_type(direction);

            match direction_type {
                DirectionType::Back => None, // 保持当前状态
                DirectionType::Side => Some(Box::new(SideRunState)),
                DirectionType::Default => Some(Box::new(RunState)),
            }
        }
    }

    fn physics_process(
        &self,
        owner: &mut Gd<CharacterBody2D>,
        resource: &mut Gd<CharacterResource>,
        _delta: f64,
    ) -> Option<Box<dyn GodotStateTraits<Owner = CharacterBody2D, Resource = CharacterResource>>>
    {
        let direction = get_input_direction();

        if direction.is_zero_approx() {
            // 如果没有输入，立即停止移动
            owner.set_velocity(Vector2::ZERO);
            return Some(Box::new(BackIdleState));
        }

        // 更新速度
        let speed = resource.bind().get_speed() * 1.5; // 跑步时速度提升
        let velocity = direction * speed as f32;
        owner.set_velocity(velocity);

        None
    }
}

// 侧向奔跑状态
#[derive(Debug)]
pub struct SideRunState;

impl CharacterStateCommon for SideRunState {
    fn get_animation_name(&self, _animation_direction: &str) -> String {
        "side_running".to_string()
    }
}

impl GodotInitialState for SideRunState {}

impl GodotState for SideRunState {
    type Owner = CharacterBody2D;
    type Resource = CharacterResource;

    fn init(&self, _owner: &mut Gd<CharacterBody2D>, resource: &mut Gd<CharacterResource>) {
        // 确保设置为side朝向
        resource.bind_mut().set_animation_direction("side");
        let animation_name = self.get_animation_name("side");
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

        if direction.is_zero_approx() {
            Some(Box::new(SideIdleState))
        } else {
            // 检查方向是否改变，需要切换到其他运动状态
            let direction_type = determine_direction_type(direction);

            match direction_type {
                DirectionType::Back => Some(Box::new(BackRunState)),
                DirectionType::Side => None, // 保持当前状态
                DirectionType::Default => Some(Box::new(RunState)),
            }
        }
    }

    fn physics_process(
        &self,
        owner: &mut Gd<CharacterBody2D>,
        resource: &mut Gd<CharacterResource>,
        _delta: f64,
    ) -> Option<Box<dyn GodotStateTraits<Owner = CharacterBody2D, Resource = CharacterResource>>>
    {
        let direction = get_input_direction();

        if direction.is_zero_approx() {
            // 如果没有输入，立即停止移动
            owner.set_velocity(Vector2::ZERO);
            return Some(Box::new(SideIdleState));
        }

        // 更新速度
        let speed = resource.bind().get_speed() * 1.5; // 跑步时速度提升
        let velocity = direction * speed as f32;
        owner.set_velocity(velocity);

        None
    }
}
