### run
- ```sudo apt install libfontconfig```
- ```sudo apt install -y mosquitto```
- ```cargo run --bin server```

### mqtt
- ```mosquitto_sub -h 45.79.116.50  -p 1883 -t "#"```

### keys mapping
```
users($email):                      <u-all, [$id_user, ...]>
                                    <u-$id_user, {
                                        password: $password_hashed,
                                        token: $token,
                                        role: $role,
                                        id_groups: [$id_group, ...],
                                    }>
groups($uuid):                      <u-$id_user, [$id_group, ...]>
                                    <g-all, [$id_group, ...]>
                                    <g-$id_group, {
                                        name: &name,
                                        id_bases: [$id_base, ...],
                                        id_templates: [$id_template, ...],
                                        id_associates: [$id_associate, ...],
                                        id_items: [$id_item, ...],
                                        id_labels: [$id_label, ...],
                                    }>
templates($keyword-$widthx$height): <g-$id_group-$id_template, {
                                        keyword: $keyword,
                                        width: $width,
                                        height: $height,
                                        thumbnail: $thumbnail,
                                        elements: [Element, ...]   
                                    }>
items($id):                         <g-$id_group-$id_item, {
                                        keyword: $keyword,
                                        width: $width,
                                        height: $height,
                                        thumbnail: $thumbnail,
                                        elements: [Element, ...]   
                                    }>
                                    <g-$id_group-$id_item-labels, [$id_label, ...]>
label($id):                         <g-$id_group-$id_template, {
                                        keyword: $keyword,
                                        width: $width,
                                        height: $height,
                                        thumbnail: $thumbnail,
                                        elements: [Element, ...]   
                                    }>
```