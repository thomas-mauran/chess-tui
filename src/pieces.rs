pub struct Pawn{}

impl Pawn{
  pub fn to_string() -> &'static str{
    return "\
    \n\
    \n\
      █\n\
     ███\n\
     ███\n\
    ";
  }
}

pub struct Rook{}

impl Rook{
  pub fn to_string() -> &'static str{
    return "\
    \n\
    █ █ █\n\
    █████\n\
     ███\n\
    █████\n\
    ";
  }
}
pub struct Queen{}

impl Queen{
  pub fn to_string() -> &'static str{
    return "\
    \n\
    █ ░ █\n\
    █ █ █\n\
     ███\n\
    █████\n\
    ";
  }
}

pub struct King{}
impl King{
  pub fn to_string() -> &'static str{
    return "\
    \n\
      █\n\
    ██░██\n\
      █\n\
    █████\n\
    ";
  }
}
pub struct Bishop{}
impl Bishop{
  pub fn to_string() -> &'static str{
    return "\
    \n\
     ███\n\
    ██ ██\n\
     ███\n\
    █████\n\
    ";
  }
}

pub struct Knight{}
impl Knight{
  pub fn to_string() -> &'static str{
    return "\
    \n\
     ██\n\
    ██░██\n\
    ███  \n\
    █████\n\
    ";
  }
}