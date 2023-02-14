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
## identifier
format = (\<Modifier\>-)\*(\<Key\>|\<Button\>)  

Modifier
* `C`: ctrl
* `S`: shift
* `M`: alt
* `W`: win  

Key
* `0-9` | `A-Z` | `F1-F12`  

Button
* `Left` | `Middle` | `Right` | `WheelUp` | `WheelDown` | `WheelLeft` | `WheelRight` | `Side1` | `Side2`
## content
action
* `set`: set magnifier factor to given `factor`.
* `add`: add magnifier factor by given `factor`. It can be negative value.
* `toggle`: if current magnifier factor is 1.0, set magnifier factor to given `factor`, else, set to 1.0.
* `exit`: exit the program.  

factor
