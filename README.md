## Fetchcat

![alt text](https://github.com/onelric/fetchcat/blob/main/img/screenshot.png?raw=true)

### Features
* Small cute cat
* Lots of distributions
* Fully written in rust
* Customizability

### Customization

#### Changing ascii
![alt text](https://github.com/onelric/fetchcat/blob/main/img/custom_ascii.png?raw=true)

Catnip will automaticall look in your ~/ folder.
```bash
catnip -f your_file
```

#### Adding padding
![alt text](https://github.com/onelric/fetchcat/blob/main/img/padding.png?raw=true)

```
catnip -p 10
```

#### Changing seperator character
![alt text](https://github.com/onelric/fetchcat/blob/main/img/seperator.png?raw=true)

This changes the character between the icons and the data.
```
catnip -s "~~"
```

### Install
Make sure cargo is installed.
```
git clone https://github.com/onelric/fetchcat.git
cd fetchcat
makepkg -si
```

