use std::simd::{LaneCount, Simd, SimdElement, SupportedLaneCount};
pub mod transforms;
pub type Position2d<T> = Simd<T, 2>;
pub type Rotator2d<T> = T;
pub type Rotation2d<T> = Rotation<T, 2>;
pub type Scaler2d<T> = Simd<T, 2>;
pub type Position3d<T> = Simd<T, 4>;
pub type Rotator3d<T> = Simd<T, 4>;
pub type Rotation3d<T> = Rotation<T, 4>;
pub type Scaler3d<T> = Simd<T, 4>;
pub struct Rotation<T: SimdElement, const N: usize>(Simd<T, N>)
where LaneCount<N>: SupportedLaneCount;
pub trait Transform2d<T>
where T: SimdElement
{
    fn set_local_position(&mut self, pos: Position2d<T>);
    fn get_local_position(&self) -> &Position2d<T>;
    fn get_mut_local_position(&mut self) -> &mut Position2d<T>;
    fn set_local_rotation(&mut self, rot: Rotation2d<T>);
    fn get_local_rotation(&self) -> &Rotation2d<T>;
    fn get_mut_local_rotation(&mut self) -> &mut Rotation2d<T>;
    fn set_local_scaler(&mut self, sca: Scaler2d<T>);
    fn get_local_scaler(&self) -> &Scaler2d<T>;
    fn get_mut_local_scaler(&mut self) -> &mut Scaler2d<T>;
}
pub trait Transform3d<T>
where T: SimdElement
{
    fn set_local_position(&mut self, pos: Position3d<T>);
    fn get_local_position(&self) -> &Position3d<T>;
    fn get_mut_local_position(&mut self) -> &mut Position3d<T>;
    fn set_local_rotation(&mut self, rot: Rotation3d<T>);
    fn get_local_rotation(&self) -> &Rotation3d<T>;
    fn get_mut_local_rotation(&mut self) -> &mut Rotation3d<T>;
    fn set_local_scaler(&mut self, sca: Scaler3d<T>);
    fn get_local_scaler(&self) -> &Scaler3d<T>;
    fn get_mut_local_scaler(&mut self) -> &mut Scaler3d<T>;
}
impl<T: SimdElement, const N: usize> Rotation<T, N>
where LaneCount<N>: SupportedLaneCount
{
    fn to_euler(&self) -> Simd<T, N> { unimplemented!() }

    fn into_euler(self) -> Simd<T, N> { unimplemented!() }

    fn from_euler(euler: Simd<T, N>) -> Self { unimplemented!() }
}
// macro_rules! impl_ {
//     {__inner(4)} => {
//         pub fn set_w(&mut self, value: T) { self.set(3, value) }
//         pub fn w(&self) -> T { self.get(3) }
//         pub fn mut_w(&mut self) -> &mut T { self.get_mut(3) }
//         pub fn set_z(&mut self, value: T) { self.set(2, value) }
//         pub fn z(&self) -> T { self.get(2) }
//         pub fn mut_z(&mut self) -> &mut T { self.get_mut(2) }
//         impl_!{__inner(2)}
//     };
//     {__inner(2)} => {
//         pub fn set_y(&mut self, value: T) { self.set(1, value) }
//         pub fn y(&self) -> T { self.get(1) }
//         pub fn mut_y(&mut self) -> &mut T { self.get_mut(1) }
//         pub fn set_x(&mut self, value: T) { self.set(0, value) }
//         pub fn x(&self) -> T { self.get(0) }
//         pub fn mut_x(&mut self) -> &mut T { self.get_mut(0) }
//     };
//     (p2) => {
//         impl<T: SimdElement> dyn Position<T, 2> {
//             impl_!{__inner(2)}
//         }
//     };
//     (p4) => {
//         impl<T: SimdElement> dyn Position<T, 4> {
//             impl_!{__inner(4)}
//         }
//     };
//     (s2) => {
//         impl<T: SimdElement> dyn Scaler<T, 2> {
//             impl_!{__inner(2)}
//         }
//     };
//     (s4) => {
//         impl<T: SimdElement> dyn Scaler<T, 4> {
//             impl_!{__inner(4)}
//         }
//     };
//     ($($idents:ident)*) => {
//         $(
//             impl_!($idents);
//         )*
//     }
// }
// impl_!(p2 p4 s2 s4);
