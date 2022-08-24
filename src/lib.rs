//! My custom Library of rust
mod macros;

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn macros_sh_cmd() {
      sh_cmd!("ls", "-a");
      sh_cmd!("ls", "-a", "--color=auto");
      sh_cmd!("ls", "-l", "-a", "--color=auto");
      sh_cmd!("ls", "-a");
      sh_cmd!("ls", { ["-l", "-a", "--color=auto"] });
      sh_cmd!("ls",);
   }

   #[test]
   fn cd_cmd() {
      sh_cmd!("builtin", "cd", "src");
      sh_cmd!("ls", { ["-a", "--color=auto"] });
   }
}
