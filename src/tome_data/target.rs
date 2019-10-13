pub enum Target {
    Matched,
    Myself,
    Query,
    TargetedBy(Box<Target>),
    Targeting(Box<Target>),
}
