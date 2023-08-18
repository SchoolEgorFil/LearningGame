```rust
enum Predicate {
    Custom(Value,Box<dyn Fn(&Value) -> bool>),
}

enum Value {
    PlayerBitStorage(u8),
}

enum Trigger {
    Hotkey(HotkeyState,Hotkey),
    Collide,
}

enum Hotkey {
    Use
    WalkFront
    WalkBack
    StrafeLeft
    StrafeRight
}

enum HotkeyState {
    JustPressed,
    Down,
    JustReleased,
    Up
}

enum BroadcastTarget {
    World,
    Entity,
    SpecialEntity_Self
}

enum WorldMessage {
    ChangeAudio(),
    
}

event PushPTBSequence(Vec<Predicate>,Vec<Trigger>,Vec<BroadcastTarget>)

event FireBroadcast { SourceEntity, TargetEntity, Option<Head>, Body }

res ResponsibilityGraph {
    delegates: HashMap<Entity to bits,Vec<Entity to bits>>
}
```

```

DoorCollider [
    [
        Trigger::HotkeyInCollisionBox(Hotkey::Use),
        [Action::new(|| {})]
    ]
]

```