
const UPPER_RMB: [&str; 10] = [
        "零", "壹", "贰", "叁", "肆", 
        "伍", "陆", "柒", "捌", "玖"
    ];
const UNIT: [&str; 6] = [
        "", "拾", "佰", "仟", "万", "亿"
    ];
const PART_UNIT: [&str; 4] = [
        "", "万", "亿", "兆"
    ];

#[napi]
pub fn rmb_to_rmb(n: f64) -> String {

    let mut u_rmb = String::new();      // Upper_RMB: 大写人民币

    
    let zs: u64 = n as u64; //整数部分

    // +0.0001保证可以防止四舍五入bug
    let xs: u64 = ((n + 0.0001) * 100.0) as u64 % 100; //小数部分

    /* 小数部分 */

    if xs == 0 {
        u_rmb.push_str("整");
    } else {
        u_rmb.push_str(decimal_part(xs).as_str());
    }

    if zs == 0 {
        if xs == 0 { u_rmb.insert_str(0, "零元") }
        return u_rmb;
    }
    
    u_rmb.insert_str(0, "元");

    u_rmb.insert_str(0, integer_part(zs).as_str());

    // u_rmb.push_str(&n.to_string()); 【用于debug】

    u_rmb
}

// 小数部分
fn decimal_part(xs: u64) -> String {
    let mut u_rmb = String::new();
    
    /* 角 */
    let _j = xs / 10 % 10;
    if _j != 0 {
        u_rmb.push_str(UPPER_RMB[_j as usize]);
        u_rmb.push_str("角");
    } else {
        u_rmb.push_str("零");
    }

    /* 分 */
    let _f = xs % 10;
    if _f != 0 {
        u_rmb.push_str(UPPER_RMB[_f as usize]);
        u_rmb.push_str("分");
    }
    
    u_rmb
}

// 整数部分
fn integer_part(zs: u64) -> String {
    if zs == 0  {return String::from("")}

    let mut u_rmb = String::new();
    let mut rmb = zs;
    let mut now = rmb % 10000;  //当前部分的数字
    let mut unit = 0; //目前单位的位置
    
    let mut last_was_zero = false;
    let mut alway_is_zero = true;  //是否从头是零
    let mut past_last_was_zero = false;  //上一个循环的首位是否是零「例如：0111」

    while unit < 4 {
        
        if now == 0 {
            unit += 1;
            rmb /= 10000;
            now = rmb % 10000;
            
            if !last_was_zero && !alway_is_zero {
                last_was_zero = true;
            }
            continue;
        } else if last_was_zero {
            u_rmb.insert_str(0, "零");
            last_was_zero = false;
        } else if !alway_is_zero && past_last_was_zero {
            u_rmb.insert_str(0, "零");
        }

        u_rmb.insert_str(0, PART_UNIT[unit as usize]);
        u_rmb.insert_str(0, integer_mid_part(now).as_str());
        alway_is_zero = false;

        if (rmb / 1000) % 10 == 0 {
            past_last_was_zero = true;
        } else {
            past_last_was_zero = false;
        }
        unit += 1;
        rmb /= 10000;
        now = rmb % 10000;
    }

    u_rmb
}


// 整数部分的4位一拆分处理
fn integer_mid_part(n: u64) -> String {
    if n == 0 { return String::from("") }

    let mut u_rmb = String::new();
    let mut unit = 0; //目前单位的位置
    let mut rmb = n; //当前部分的数字
    let mut now = n % 10;  //当前位的数字

    let mut last_was_zero = false; //是否需要加零
    let mut is_alway_zero = true;  //是否从头是零

    while unit < 4 && rmb > 0 {
        if now == 0 {
            unit += 1;
            rmb /= 10;
            now = rmb % 10;
            
            if !last_was_zero && !is_alway_zero {
                last_was_zero = true; // 标记下次循环的上次（即本次）为零
            }
            continue;

        } else if last_was_zero {
            u_rmb.insert_str(0, "零");
            last_was_zero = false;
        }

        u_rmb.insert_str(0, UNIT[unit as usize]);
        u_rmb.insert_str(0, UPPER_RMB[now as usize]);

        is_alway_zero = false;

        unit += 1;
        rmb /= 10;
        now = rmb % 10;
    }


    u_rmb
}