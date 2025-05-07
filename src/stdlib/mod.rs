mod events;
mod utils;
use crate::prelude::*;

pub fn load_stdlib(rt: &mut Runtime) {
    let print = include_str!("utils/print.ron");
    let ron = ron::from_str(print).unwrap();
    rt.register_bead("stdlib/utils/print".into(), ron);
}

#[cfg(test)]
mod test {
    #[test]
    fn load_stdlib_test() {
        use crate::prelude::*;
        let (mut rt, tx) = Runtime::new();

        crate::stdlib::load_stdlib(&mut rt);
        crate::utils::to_file(format!("{:#?}", rt), "stdlib_rt.ron");
    }
}
