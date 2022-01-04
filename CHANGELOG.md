## 2.2.0 (January 2, 2022)

### Release 2.2.0
* New Feature: 
  * cursor ***`.prev()`*** : *next_back() without turnaround().*
  * cursor ***`.extras_mut()`*** : *gets mutable extras.*
  

---

## 2.1.0 (January 2, 2022)

### Release 2.1.0
* Patched:
    - change(&mut self, input: &char`, pos: usize`) in `Extras`
* New Feature: 
    - *.next_to_left()*
    - *.next_to_right()*
  * *If `next` or `jump` can effect the `Extras`.*
    - *.noeffects()*
    - *.noeffects_mut()*
    - *.noeffects_on()*
    - *.noefeects_off()*  
  * *Bump until meets `fn` = `true`.*
    - *.next_to_until(`fn`)*
  * *Bump while `fn` = `true`.*
    - *.next_to_while(`fn`)*
  * *Cloning `saved().extras` to `self.extras()`.*
    - *.to_range_extras()*
---

## 2.0.0 (December 29, 2021)

### Release 2.0.0
* Changed whole structure.
* New Features: 
  - *Please check this [diagram](README.md)*

---

## 1.2.0 (December 20, 2021)

### Release 1.2.0
* New Feature: 
  - *cursor.save();*
  - // *...*
  - let extras = *cursor.load_extras();*

---

## 1.1.0 (December 19, 2021)

### Release 1.1.0
* New Features: 
  - *Cursor::new_with_extras::<`Extras`>(*`slice`*);*
  - *StrCursor::new_with_extras::<`Extras`>(*`str`*);*

---

## 1.0.0 (December 19, 2021)

### Release 1.0.0

---