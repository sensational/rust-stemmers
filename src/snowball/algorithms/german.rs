//! Generated by Snowball 2.2.0 - https://snowballstem.org/

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
use snowball::SnowballEnv;
use snowball::Among;

static A_0: &'static [Among<Context>; 6] = &[
    Among("", -1, 5, None),
    Among("U", 0, 2, None),
    Among("Y", 0, 1, None),
    Among("\u{00E4}", 0, 3, None),
    Among("\u{00F6}", 0, 4, None),
    Among("\u{00FC}", 0, 2, None),
];

static A_1: &'static [Among<Context>; 7] = &[
    Among("e", -1, 2, None),
    Among("em", -1, 1, None),
    Among("en", -1, 2, None),
    Among("ern", -1, 1, None),
    Among("er", -1, 1, None),
    Among("s", -1, 3, None),
    Among("es", 5, 2, None),
];

static A_2: &'static [Among<Context>; 4] = &[
    Among("en", -1, 1, None),
    Among("er", -1, 1, None),
    Among("st", -1, 2, None),
    Among("est", 2, 1, None),
];

static A_3: &'static [Among<Context>; 2] = &[
    Among("ig", -1, 1, None),
    Among("lich", -1, 1, None),
];

static A_4: &'static [Among<Context>; 8] = &[
    Among("end", -1, 1, None),
    Among("ig", -1, 2, None),
    Among("ung", -1, 1, None),
    Among("lich", -1, 3, None),
    Among("isch", -1, 2, None),
    Among("ik", -1, 2, None),
    Among("heit", -1, 3, None),
    Among("keit", -1, 4, None),
];

static G_v: &'static [u8; 20] = &[17, 65, 16, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 32, 8];

static G_s_ending: &'static [u8; 3] = &[117, 30, 5];

static G_st_ending: &'static [u8; 3] = &[117, 30, 4];

#[derive(Clone)]
struct Context {
    i_x: i32,
    i_p2: i32,
    i_p1: i32,
}

fn r_prelude(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let v_1 = env.cursor;
    'replab0: loop{
        let v_2 = env.cursor;
        'lab1: for _ in 0..1 {
            'lab2: loop {
                let v_3 = env.cursor;
                'lab3: loop {
                    env.bra = env.cursor;
                    if !env.eq_s(&"\u{00DF}") {
                        break 'lab3;
                    }
                    env.ket = env.cursor;
                    if !env.slice_from("ss") {
                        return false;
                    }
                    break 'lab2;
                }
                env.cursor = v_3;
                if env.cursor >= env.limit {
                    break 'lab1;
                }
                env.next_char();
                break 'lab2;
            }
            continue 'replab0;
        }
        env.cursor = v_2;
        break 'replab0;
    }
    env.cursor = v_1;
    'replab4: loop{
        let v_4 = env.cursor;
        'lab5: for _ in 0..1 {
            'golab6: loop {
                let v_5 = env.cursor;
                'lab7: loop {
                    if !env.in_grouping(G_v, 97, 252) {
                        break 'lab7;
                    }
                    env.bra = env.cursor;
                    'lab8: loop {
                        let v_6 = env.cursor;
                        'lab9: loop {
                            if !env.eq_s(&"u") {
                                break 'lab9;
                            }
                            env.ket = env.cursor;
                            if !env.in_grouping(G_v, 97, 252) {
                                break 'lab9;
                            }
                            if !env.slice_from("U") {
                                return false;
                            }
                            break 'lab8;
                        }
                        env.cursor = v_6;
                        if !env.eq_s(&"y") {
                            break 'lab7;
                        }
                        env.ket = env.cursor;
                        if !env.in_grouping(G_v, 97, 252) {
                            break 'lab7;
                        }
                        if !env.slice_from("Y") {
                            return false;
                        }
                        break 'lab8;
                    }
                    env.cursor = v_5;
                    break 'golab6;
                }
                env.cursor = v_5;
                if env.cursor >= env.limit {
                    break 'lab5;
                }
                env.next_char();
            }
            continue 'replab4;
        }
        env.cursor = v_4;
        break 'replab4;
    }
    return true
}

fn r_mark_regions(env: &mut SnowballEnv, context: &mut Context) -> bool {
    context.i_p1 = env.limit;
    context.i_p2 = env.limit;
    let v_1 = env.cursor;
    if !env.hop(3) {
        return false;
    }
    context.i_x = env.cursor;
    env.cursor = v_1;
    'golab0: loop {
        'lab1: loop {
            if !env.in_grouping(G_v, 97, 252) {
                break 'lab1;
            }
            break 'golab0;
        }
        if env.cursor >= env.limit {
            return false;
        }
        env.next_char();
    }
    'golab2: loop {
        'lab3: loop {
            if !env.out_grouping(G_v, 97, 252) {
                break 'lab3;
            }
            break 'golab2;
        }
        if env.cursor >= env.limit {
            return false;
        }
        env.next_char();
    }
    context.i_p1 = env.cursor;
    'lab4: loop {
        if context.i_p1 >= context.i_x{
            break 'lab4;
        }
        context.i_p1 = context.i_x;
        break 'lab4;
    }
    'golab5: loop {
        'lab6: loop {
            if !env.in_grouping(G_v, 97, 252) {
                break 'lab6;
            }
            break 'golab5;
        }
        if env.cursor >= env.limit {
            return false;
        }
        env.next_char();
    }
    'golab7: loop {
        'lab8: loop {
            if !env.out_grouping(G_v, 97, 252) {
                break 'lab8;
            }
            break 'golab7;
        }
        if env.cursor >= env.limit {
            return false;
        }
        env.next_char();
    }
    context.i_p2 = env.cursor;
    return true
}

fn r_postlude(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    'replab0: loop{
        let v_1 = env.cursor;
        'lab1: for _ in 0..1 {
            env.bra = env.cursor;
            among_var = env.find_among(A_0, context);
            env.ket = env.cursor;
            match among_var {
                1 => {
                    if !env.slice_from("y") {
                        return false;
                    }
                }
                2 => {
                    if !env.slice_from("u") {
                        return false;
                    }
                }
                3 => {
                    if !env.slice_from("a") {
                        return false;
                    }
                }
                4 => {
                    if !env.slice_from("o") {
                        return false;
                    }
                }
                5 => {
                    if env.cursor >= env.limit {
                        break 'lab1;
                    }
                    env.next_char();
                }
                _ => ()
            }
            continue 'replab0;
        }
        env.cursor = v_1;
        break 'replab0;
    }
    return true
}

fn r_R1(env: &mut SnowballEnv, context: &mut Context) -> bool {
    return context.i_p1 <= env.cursor
}

fn r_R2(env: &mut SnowballEnv, context: &mut Context) -> bool {
    return context.i_p2 <= env.cursor
}

fn r_standard_suffix(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    let v_1 = env.limit - env.cursor;
    'lab0: loop {
        env.ket = env.cursor;
        among_var = env.find_among_b(A_1, context);
        if among_var == 0 {
            break 'lab0;
        }
        env.bra = env.cursor;
        if !r_R1(env, context) {
            break 'lab0;
        }
        match among_var {
            1 => {
                if !env.slice_del() {
                    return false;
                }
            }
            2 => {
                if !env.slice_del() {
                    return false;
                }
                let v_2 = env.limit - env.cursor;
                'lab1: loop {
                    env.ket = env.cursor;
                    if !env.eq_s_b(&"s") {
                        env.cursor = env.limit - v_2;
                        break 'lab1;
                    }
                    env.bra = env.cursor;
                    if !env.eq_s_b(&"nis") {
                        env.cursor = env.limit - v_2;
                        break 'lab1;
                    }
                    if !env.slice_del() {
                        return false;
                    }
                    break 'lab1;
                }
            }
            3 => {
                if !env.in_grouping_b(G_s_ending, 98, 116) {
                    break 'lab0;
                }
                if !env.slice_del() {
                    return false;
                }
            }
            _ => ()
        }
        break 'lab0;
    }
    env.cursor = env.limit - v_1;
    let v_3 = env.limit - env.cursor;
    'lab2: loop {
        env.ket = env.cursor;
        among_var = env.find_among_b(A_2, context);
        if among_var == 0 {
            break 'lab2;
        }
        env.bra = env.cursor;
        if !r_R1(env, context) {
            break 'lab2;
        }
        match among_var {
            1 => {
                if !env.slice_del() {
                    return false;
                }
            }
            2 => {
                if !env.in_grouping_b(G_st_ending, 98, 116) {
                    break 'lab2;
                }
                if !env.hop_back(3) {
                    break 'lab2;
                }
                if !env.slice_del() {
                    return false;
                }
            }
            _ => ()
        }
        break 'lab2;
    }
    env.cursor = env.limit - v_3;
    let v_4 = env.limit - env.cursor;
    'lab3: loop {
        env.ket = env.cursor;
        among_var = env.find_among_b(A_4, context);
        if among_var == 0 {
            break 'lab3;
        }
        env.bra = env.cursor;
        if !r_R2(env, context) {
            break 'lab3;
        }
        match among_var {
            1 => {
                if !env.slice_del() {
                    return false;
                }
                let v_5 = env.limit - env.cursor;
                'lab4: loop {
                    env.ket = env.cursor;
                    if !env.eq_s_b(&"ig") {
                        env.cursor = env.limit - v_5;
                        break 'lab4;
                    }
                    env.bra = env.cursor;
                    let v_6 = env.limit - env.cursor;
                    'lab5: loop {
                        if !env.eq_s_b(&"e") {
                            break 'lab5;
                        }
                        env.cursor = env.limit - v_5;
                        break 'lab4;
                    }
                    env.cursor = env.limit - v_6;
                    if !r_R2(env, context) {
                        env.cursor = env.limit - v_5;
                        break 'lab4;
                    }
                    if !env.slice_del() {
                        return false;
                    }
                    break 'lab4;
                }
            }
            2 => {
                let v_7 = env.limit - env.cursor;
                'lab6: loop {
                    if !env.eq_s_b(&"e") {
                        break 'lab6;
                    }
                    break 'lab3;
                }
                env.cursor = env.limit - v_7;
                if !env.slice_del() {
                    return false;
                }
            }
            3 => {
                if !env.slice_del() {
                    return false;
                }
                let v_8 = env.limit - env.cursor;
                'lab7: loop {
                    env.ket = env.cursor;
                    'lab8: loop {
                        let v_9 = env.limit - env.cursor;
                        'lab9: loop {
                            if !env.eq_s_b(&"er") {
                                break 'lab9;
                            }
                            break 'lab8;
                        }
                        env.cursor = env.limit - v_9;
                        if !env.eq_s_b(&"en") {
                            env.cursor = env.limit - v_8;
                            break 'lab7;
                        }
                        break 'lab8;
                    }
                    env.bra = env.cursor;
                    if !r_R1(env, context) {
                        env.cursor = env.limit - v_8;
                        break 'lab7;
                    }
                    if !env.slice_del() {
                        return false;
                    }
                    break 'lab7;
                }
            }
            4 => {
                if !env.slice_del() {
                    return false;
                }
                let v_10 = env.limit - env.cursor;
                'lab10: loop {
                    env.ket = env.cursor;
                    if env.find_among_b(A_3, context) == 0 {
                        env.cursor = env.limit - v_10;
                        break 'lab10;
                    }
                    env.bra = env.cursor;
                    if !r_R2(env, context) {
                        env.cursor = env.limit - v_10;
                        break 'lab10;
                    }
                    if !env.slice_del() {
                        return false;
                    }
                    break 'lab10;
                }
            }
            _ => ()
        }
        break 'lab3;
    }
    env.cursor = env.limit - v_4;
    return true
}

pub fn stem(env: &mut SnowballEnv) -> bool {
    let mut context = &mut Context {
        i_x: 0,
        i_p2: 0,
        i_p1: 0,
    };
    let v_1 = env.cursor;
    r_prelude(env, context);
    env.cursor = v_1;
    let v_2 = env.cursor;
    r_mark_regions(env, context);
    env.cursor = v_2;
    env.limit_backward = env.cursor;
    env.cursor = env.limit;
    r_standard_suffix(env, context);
    env.cursor = env.limit_backward;
    let v_4 = env.cursor;
    r_postlude(env, context);
    env.cursor = v_4;
    return true
}
