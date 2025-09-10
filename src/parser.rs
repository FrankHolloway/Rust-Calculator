pub fn TakeNewInput(_newString: &str) -> Vec<i32>{
    return Split(_newString.trim().as_bytes());
}

fn Split(_byteArray: &[u8]) -> Vec<i32>{
    let mut distance: i16 = 0;
    let mut numOfSections: i16 = 0;
    let mut sections: Vec<section> = Vec::new();

    for i in 0.._byteArray.len(){
        if _byteArray[i] == b' '{
            continue;
        }else if _byteArray[i] == 44 {
            numOfSections += 1;
            let new_sec  = section {
                name: format!("sec{}", numOfSections),
                start: i as i16 - distance, 
                length: distance, 
            };
            sections.push(new_sec);
            distance = 0;
        }
        else{
            distance = &distance + 1;
        }

        if i == _byteArray.len() - 1 {
            numOfSections += 1;
            let new_sec = section {
                name: format!("sec{}", numOfSections),
                start: (i as i16) - distance + 1,
                length: distance,
            };
            sections.push(new_sec);
        }
    }

    let mut setVector = Vec::new();

    for section in sections {
        //println!("Name: {}, Start: {}, Length: {}", section.name, section.start, section.length);

        let mut outputNum: f64 = 0.0;

        let mut findingDecimal = false;
        let mut decimalPlacement: i16 = 0;

        for i in section.start..(section.start+section.length){

            let mut numFoundInLoop = 0;

            match _byteArray[i as usize]{
                48 => numFoundInLoop = 0,
                49 => numFoundInLoop = 1,
                50 => numFoundInLoop = 2,
                51 => numFoundInLoop = 3,
                52 => numFoundInLoop = 4,
                53 => numFoundInLoop = 5,
                54 => numFoundInLoop = 6,
                55 => numFoundInLoop = 7,
                56 => numFoundInLoop = 8,
                57 => numFoundInLoop = 9,
                _ => continue,
            }   
            
            let power = if findingDecimal {
                decimalPlacement - i
            } else {
                (section.length - 1) - (i - section.start)
            };


            
                outputNum += numFoundInLoop as f64 * (10_i32).pow(power as u32) as f64;
            
        }

        setVector.push(outputNum as i32);
    }

    return setVector;
}

struct section {
    name: String,
    start: i16,
    length: i16,
}
