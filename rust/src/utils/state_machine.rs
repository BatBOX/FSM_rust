use godot::classes::InputEvent;
use godot::prelude::*;

pub trait GodotStateTraits: GodotState + Sync + Send + std::any::Any + std::fmt::Debug {
    /// 返回`self` as `&mut dyn Any`
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

/// 状态是一个虚拟基类，允许结构在状态机中用作状态。
pub trait GodotState: std::fmt::Debug {
    type Owner: GodotClass + Inherits<Node>;
    type Resource: GodotClass;

    /// 虚拟功能。更改活动状态后，由状态机器调用
    fn init(&self, _owner: &mut Gd<Self::Owner>, _resource: &mut Gd<Self::Resource>) {}

    /// 虚拟函数。对应于`_ready()`回调
    fn ready(&self, _owner: &mut Gd<Self::Owner>, _resource: &mut Gd<Self::Resource>) {}

    /// 虚拟函数。对应`_input()`回调
    fn input(
        &self,
        _owner: &Gd<Self::Owner>,
        _resource: &mut Gd<Self::Resource>,
        _event: Gd<InputEvent>,
    ) -> Option<Box<dyn GodotStateTraits<Owner = Self::Owner, Resource = Self::Resource>>> {
        None
    }

    /// 虚拟函数。对应`_process()`回调
    fn process(
        &self,
        _owner: &mut Gd<Self::Owner>,
        _resource: &mut Gd<Self::Resource>,
        _delta: f64,
    ) -> Option<Box<dyn GodotStateTraits<Owner = Self::Owner, Resource = Self::Resource>>> {
        None
    }

    /// 虚拟函数。对应`_physics_process()`回调
    fn physics_process(
        &self,
        _owner: &mut Gd<Self::Owner>,
        _resource: &mut Gd<Self::Resource>,
        _delta: f64,
    ) -> Option<Box<dyn GodotStateTraits<Owner = Self::Owner, Resource = Self::Resource>>> {
        None
    }

    /// 虚拟函数。对应`_integrate_forces()`回调
    fn integrate_forces(
        &self,
        _owner: &Gd<Self::Owner>,
        _resource: &mut Gd<Self::Resource>,
        _delta: f64,
    ) {
    }
}

// 允许状态机以类型安全的方式处理各种不同的状态类型，同时保留在运行时检查和转换状态类型的能力
impl<T> GodotStateTraits for T
where
    T: GodotState + std::any::Any + Sync + Send + std::fmt::Debug,
{
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// 初始状态是一种自定义标记性状，允许状态被用作Machine中的初始状态。
pub trait GodotInitialState: GodotStateTraits {}

/// Machine提供查询状态机的当前状态所需的方法。
pub trait GodotMachine: std::fmt::Debug {
    type Owner: GodotClass + Inherits<Node>;
    type Resource: GodotClass;

    /// 允许您初始化Machine的当前状态。
    fn init(base: Base<Node>) -> Self;

    /// 允许您更新Machine的当前状态。
    fn state(
        &mut self,
        state: Box<dyn GodotStateTraits<Owner = Self::Owner, Resource = Self::Resource>>,
    );
}

/// 构造并返回一台新状态机
pub trait GodotInitializer {
    type Owner: GodotClass + Inherits<Node>;
    type Resource: GodotClass;

    /// 新的初始化一台新机器，基于提供的"GodotInitialState"作为输入。
    fn new(state: impl GodotInitialState<Owner = Self::Owner, Resource = Self::Resource>) -> Self;
}
