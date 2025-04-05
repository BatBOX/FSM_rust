use crate::player::states_impl::idle_state::IdleState;
use crate::utils::character_state_common::CharacterResource;
use crate::utils::state_machine::{GodotMachine, GodotStateTraits};
use godot::classes::{CharacterBody2D, InputEvent, Node};
use godot::prelude::*;

/// 玩家状态机，实现GodotMachine和GodotInitializer特性
#[derive(GodotClass, Debug)]
#[class(base=Node)]
pub struct PlayerStateMachine {
    current_state:
        Option<Box<dyn GodotStateTraits<Owner = CharacterBody2D, Resource = CharacterResource>>>,
    resource: Gd<CharacterResource>,
    owner: Gd<CharacterBody2D>,
    base: Base<Node>,
}

impl GodotMachine for PlayerStateMachine {
    type Owner = CharacterBody2D;
    type Resource = CharacterResource;

    fn init(base: Base<Node>) -> Self {
        let resource = CharacterResource::new();
        Self {
            current_state: None,
            owner: CharacterBody2D::new_alloc(),
            resource,
            base,
        }
    }

    fn state(
        &mut self,
        state: Box<dyn GodotStateTraits<Owner = Self::Owner, Resource = Self::Resource>>,
    ) {
        // 保存新状态
        self.current_state = Some(state);

        // 初始化新状态
        if let Some(state) = &self.current_state {
            state.init(&mut self.owner(), &mut self.resource);
        }
    }
}

#[godot_api]
impl INode for PlayerStateMachine {
    fn init(base: Base<Node>) -> Self {
        GodotMachine::init(base)
    }

    fn process(&mut self, delta: f64) {
        if let Some(parent) = self.base().get_parent() {
            if let Ok(player) = parent.try_cast::<CharacterBody2D>() {
                self.owner = player.clone();
                self.handle_process(delta);
            }
        }
    }

    fn physics_process(&mut self, delta: f64) {
        if let Some(parent) = self.base().get_parent() {
            if let Ok(player) = parent.try_cast::<CharacterBody2D>() {
                self.owner = player.clone();
                self.handle_physics_process(delta);
            }
        }
    }

    fn ready(&mut self) {
        // 获取父节点（玩家角色）
        if let Some(parent) = self.base().get_parent() {
            if let Ok(player) = parent.try_cast::<CharacterBody2D>() {
                self.owner = player.clone();

                // 确保资源已经正确设置（AnimationPlayer等）
                // 这一步很重要，确保在初始化状态前资源已准备好
                let animation_player = self.owner.get_node_as("AnimationPlayer");
                self.resource
                    .bind_mut()
                    .set_animation_player(animation_player);

                // 设置初始状态为IdleState（如果尚未设置状态）
                if self.current_state.is_none() {
                    self.current_state = Some(Box::new(IdleState));
                }

                // 初始化状态
                if let Some(state) = &self.current_state {
                    state.init(&mut self.owner, &mut self.resource);
                }
            } else {
                self.owner = CharacterBody2D::new_alloc();
            }
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if let Some(parent) = self.base().get_parent() {
            if let Ok(player) = parent.try_cast::<CharacterBody2D>() {
                self.owner = player.clone();
                self.handle_input(event);
            }
        }
    }
}

impl PlayerStateMachine {
    /// 获取当前状态机的所有者（玩家角色）
    pub fn owner(&self) -> Gd<CharacterBody2D> {
        // 从父节点获取玩家角色
        if let Some(parent) = self.base().get_parent() {
            if let Ok(player) = parent.try_cast::<CharacterBody2D>() {
                return player;
            }
        }
        // 如果无法获取，返回一个新的实例（这种情况应该很少发生）
        CharacterBody2D::new_alloc()
    }

    /// 设置资源
    pub fn set_resource(&mut self, resource: Gd<CharacterResource>) {
        self.resource = resource;
    }

    /// 获取资源
    pub fn resource(&self) -> &Gd<CharacterResource> {
        &self.resource
    }

    /// 获取资源（可变）
    pub fn resource_mut(&mut self) -> &mut Gd<CharacterResource> {
        &mut self.resource
    }

    /// 处理输入事件
    pub fn handle_input(&mut self, event: Gd<InputEvent>) {
        let mut owner = self.owner();
        if let Some(state) = &self.current_state {
            if let Some(new_state) = state.input(&mut owner, &mut self.resource, event) {
                self.state(new_state);
            }
        }
    }

    /// 处理更新事件
    pub fn handle_process(&mut self, delta: f64) {
        let mut owner = self.owner();
        if let Some(state) = &self.current_state {
            if let Some(new_state) = state.process(&mut owner, &mut self.resource, delta) {
                self.state(new_state);
            }
        }
    }

    /// 处理物理更新事件（60s）
    pub fn handle_physics_process(&mut self, delta: f64) {
        let mut owner = self.owner();
        if let Some(state) = &self.current_state {
            if let Some(new_state) = state.physics_process(&mut owner, &mut self.resource, delta) {
                self.state(new_state);
            }
        }
    }

    /// 处理处理物理受力事件
    pub fn handle_integrate_forces(&mut self, owner: &Gd<CharacterBody2D>, delta: f64) {
        if let Some(state) = &self.current_state {
            state.integrate_forces(owner, &mut self.resource, delta);
        }
    }

    /// 获取当前状态的类型名称（用于调试）
    pub fn current_state_name(&self) -> String {
        if let Some(state) = &self.current_state {
            godot_print!("Current State: {:?}", state);
            format!("{:?}", state)
        } else {
            "None".to_string()
        }
    }
}
