use crate::{MenuIndexFromScreenCoord, MenuInstance, MenuOutput};
use prototty_input::Input;
use prototty_render::{Frame, View, ViewContext, ViewTransformRgb24};
use prototty_tick_routine::{Tick, TickRoutine, ViewSelector};
use std::marker::PhantomData;
use std::time::Duration;

pub struct MenuInstanceRoutine<C, V> {
    choice: PhantomData<C>,
    view: PhantomData<V>,
}
impl<C, V> MenuInstanceRoutine<C, V>
where
    C: Clone,
    for<'a> V: View<&'a MenuInstance<C>>,
{
    fn new() -> Self {
        Self {
            choice: PhantomData,
            view: PhantomData,
        }
    }
}
impl<C, V> Clone for MenuInstanceRoutine<C, V>
where
    C: Clone,
    for<'a> V: View<&'a MenuInstance<C>>,
{
    fn clone(&self) -> Self {
        Self::new()
    }
}
impl<C, V> Copy for MenuInstanceRoutine<C, V>
where
    C: Clone,
    for<'a> V: View<&'a MenuInstance<C>>,
{
}

impl<C, V> TickRoutine for MenuInstanceRoutine<C, V>
where
    C: Clone,
    for<'a> V: View<&'a MenuInstance<C>>,
    V: MenuIndexFromScreenCoord,
{
    type Return = MenuOutput<C>;
    type Data = MenuInstance<C>;
    type View = V;

    fn tick<I>(
        self,
        data: &mut Self::Data,
        inputs: I,
        view: &Self::View,
        _duration: Duration,
    ) -> Tick<Self::Return, Self>
    where
        I: Iterator<Item = Input>,
    {
        if let Some(menu_output) = data.tick_with_mouse(inputs, view) {
            Tick::Return(menu_output)
        } else {
            Tick::Continue(self)
        }
    }
    fn view<F, R>(&self, data: &Self::Data, view: &mut Self::View, context: ViewContext<R>, frame: &mut F)
    where
        F: Frame,
        R: ViewTransformRgb24,
    {
        view.view(&data, context, frame);
    }
}

pub trait MenuInstanceExtraSelect {
    type DataInput;
    type Choice: Clone;
    type Extra;
    fn menu_instance<'a>(&self, input: &'a Self::DataInput) -> &'a MenuInstance<Self::Choice>;
    fn menu_instance_mut<'a>(&self, input: &'a mut Self::DataInput) -> &'a mut MenuInstance<Self::Choice>;
    fn extra<'a>(&self, input: &'a Self::DataInput) -> &'a Self::Extra;
}

pub struct MenuInstanceExtraRoutine<S> {
    s: S,
}
impl<S> MenuInstanceExtraRoutine<S>
where
    S: MenuInstanceExtraSelect,
{
    pub fn new(s: S) -> Self {
        Self { s }
    }
}

impl<S> TickRoutine for MenuInstanceExtraRoutine<S>
where
    S: MenuInstanceExtraSelect + ViewSelector,
    S::ViewOutput: MenuIndexFromScreenCoord,
    for<'a> S::ViewOutput: View<(&'a MenuInstance<S::Choice>, &'a S::Extra)>,
{
    type Return = MenuOutput<S::Choice>;
    type Data = S::DataInput;
    type View = S::ViewInput;

    fn tick<I>(
        self,
        data: &mut Self::Data,
        inputs: I,
        view: &Self::View,
        _duration: Duration,
    ) -> Tick<Self::Return, Self>
    where
        I: Iterator<Item = Input>,
    {
        let menu_instance = self.s.menu_instance_mut(data);
        let menu_view = self.s.view(view);
        if let Some(menu_output) = menu_instance.tick_with_mouse(inputs, menu_view) {
            Tick::Return(menu_output)
        } else {
            Tick::Continue(self)
        }
    }
    fn view<F, R>(&self, data: &Self::Data, view: &mut Self::View, context: ViewContext<R>, frame: &mut F)
    where
        F: Frame,
        R: ViewTransformRgb24,
    {
        let menu_view = self.s.view_mut(view);
        let menu_instance = self.s.menu_instance(data);
        let extra = self.s.extra(data);
        menu_view.view((menu_instance, extra), context, frame)
    }
}
