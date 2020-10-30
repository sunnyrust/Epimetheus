//! # 这个工具是为了实现各种基础工具
//! 
pub mod sunny_log{
    use chrono::{Local, NaiveDateTime};
    extern crate colored; // not needed in Rust 2018
    use std::thread;
    use colored::*;
    // use std::time::Duration;
    #[allow(dead_code)]

    ///print 打印
    /// # Examples
    ///  info("测试信息".to_string());
    /// 这个实际调用的是
    /// print("测试信息".to_string(),"info");
    fn log_print(msg: String,symbol:&str)  {
        let now: NaiveDateTime = Local::now().naive_local();
        let mut symbol_color="【I】".blue().bold();
        if symbol=="error"{
            symbol_color="【E】".red().bold();
        }else if symbol=="warning"{
            symbol_color="【W】".yellow().bold();
        }else if symbol=="debug"{
            symbol_color="【D】".color("white").on_color("blue").bold();
        }
        eprintln!("{} {}{}",now.format("%Y%m%d-%H:%M:%S%.f").to_string(), msg,symbol_color);
    }
    #[allow(unused_must_use)]
    pub fn info(msg: String){
        thread::spawn(|| { 
           log_print(msg,"info");
        });
    }

    pub fn error(msg: String){
        thread::spawn(|| { 
           log_print(msg,"error");
        });
    }

    pub fn warning(msg: String){
        thread::spawn(|| { 
           log_print(msg,"warning");
        });
    }


    pub fn debug(msg: String){
        thread::spawn(|| { 
           log_print(msg,"debug");
        });
    }
}