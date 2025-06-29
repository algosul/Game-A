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

    fn from_euler(_euler: Simd<T, N>) -> Self { unimplemented!() }
}
