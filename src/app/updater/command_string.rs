use anyhow::anyhow;
use anyhow::Result;

pub fn convert(src: &str) -> Result<String> {
    let mut res = String::new();
    let mut mult: Option<u32> = None;

    for ch in src.chars() {
        match ch {
            '\n' | '\r' | '\t' | ' ' => mult = None,
            '0'..='9' => {
                let digit: u32 = (ch as u32) - 48;
                if let Some(prev_mult) = mult {
                    mult = Some(digit + prev_mult * 10);
                } else {
                    mult = Some(digit);
                }
            }
            'h' | 'j' | 'k' | 'l' => {
                for _ in 0..mult.unwrap_or(1) {
                    res.push(ch);
                }
                mult = None;
            }
            c @ _ => return Err(anyhow!("unsupported character: <{}>", c)),
        };
    }
    Ok(res)
}

//  //  //  //  //  //  //  //  //  //
//          TEST                    //
//  //  //  //  //  //  //  //  //  //
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ignoring() -> Result<()> {
        assert!((convert("\n3\n44\n\t66\t 432 \r90\r  ")?).is_empty());
        Ok(())
    }

    #[test]
    fn simple_moving() -> Result<()> {
        let src = "hjkl1234567890";
        let res = convert(src)?;

        assert!(res == "hjkl");

        Ok(())
    }

    #[test]
    fn simple_moving_2() -> Result<()> {
        let src = "hjkl0h0j0k0l";
        let res = convert(src)?;

        assert!(res == "hjkl");

        Ok(())
    }

    #[test]
    fn ext_moving() -> Result<()> {
        let src = "hjkl1h1j1k1l2h2j2k2l";
        let res = convert(src)?;

        assert!(res == "hjklhjklhhjjkkll");

        Ok(())
    }

    #[test]
    fn ext_moving_2() -> Result<()> {
        let src = "12h13j11k15l";
        let res = convert(src)?;

        assert!(res == "hhhhhhhhhhhhjjjjjjjjjjjjjkkkkkkkkkkklllllllllllllll");

        Ok(())
    }

    #[test]
    fn should_error() -> Result<()> {
        assert!(convert("y1h2j\n3k55l").is_err());

        Ok(())
    }
}
