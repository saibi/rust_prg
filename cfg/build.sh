#!/bin/bash 

rustc custom.rs
rustc --cfg some_condition --cfg my_cfg=\"test\" custom.rs
./custom
