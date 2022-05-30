# AreaEx

## Construction

Should be fairly shopisticated. Reference syntax:

```
let area_gen = AreaGen::new()
area_gen.begin_group()
            .dir(up)
            .dir(down)
            .dir(left)
        .end_group()
    .or()
        .times(3)
        .dir(right);
```

Should be serializable. Could be built with a tool.

## Evaluation

Given a sequence of movement directions, the AreaEx should return whether it matches.

## Next state

Given a sequence of movement directions, the AreaEx should return the next valid directions which the sequence could take. Because of this, there won't be the *any* direction.