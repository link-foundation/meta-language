`SNode::Let` preserves omitted `type` fields, explicit null annotations, and
concrete types separately. Its `ty` field is now
`Option<SurfaceBindingTypeAnnotation>`: use `None` for omission,
`Some(SurfaceBindingTypeAnnotation::null())` for null, and `Some(ty.into())`
for a concrete type. Convert an existing `Option<Type>` with `.map(Into::into)`;
borrow a present annotation's type with `.as_ref()`.
