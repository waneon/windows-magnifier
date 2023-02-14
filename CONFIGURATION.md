# config.yaml
configuration file must be located in the path below.  
`%LOCALAPPDATA%/windows-magnifier/config.yaml`

# Example
```sh
shortcut:
  Side1:
    action: set
    factor: 1.0
  Side2:
    action: add
    factor: 0.5
  M-1:
    action: add
    factor: 0.5
  M-2:
    action: set
    factor: 1.0
  M-3:
    action: exit
```

# shortcut
format = Identifier: Content
Identifier = (\<Modifier\>-)\*(\<Key\>|\<Button\>)
Modifier = C(ctrl) | S(shift) | M(alt) | W(win)
Key = A-Z | 0-9 | F1-F12
Button = Left | Middle | Right | WheelUp | WheelDown | WheelLeft | WheelRight | Side1 | Side2
