ROS 2 for Rust
==============
```shell
## Enable shared memory with
export CYCLONEDDS_URI=file://$PWD/cyclonedds.xml
export RMW_IMPLEMENTATION=rmw_cyclonedds_cpp
iox-roudi -c roudi_config.toml &


# In a new terminal (or tmux window)
. ./install/setup.sh
cargo run examples_rclrs_minimal_pub_sub minimal_publisher
# In a new terminal (or tmux window)
. ./install/setup.sh
cargo run examples_rclrs_minimal_pub_sub minimal_subscriber
```
