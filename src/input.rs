use std::{io, str::FromStr, error::Error};

pub fn get_line() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
}

pub fn get_number<T>() -> Result<T, Box<dyn Error + 'static>>
where 
    T: FromStr,
    <T as FromStr>::Err: Error + 'static
{
    Ok(get_line()?.trim().parse::<T>()?)
}

pub fn get_vector<T>() -> Result<Vec<T>, Box<dyn Error + 'static>>
where 
    T: FromStr,
    <T as FromStr>::Err: Error + 'static
{
    Ok(get_line()?.split_whitespace().map(|c| c.parse::<T>()).collect::<Result<Vec<T>, _>>()?)
}

pub fn get_unspace_vector() -> Option<Vec<u32>> { 
    Some(get_line().ok()?.trim().chars().map(|c| c.to_digit(10)).collect::<Option<Vec<u32>>>()?)
}
