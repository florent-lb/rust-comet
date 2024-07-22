use std::io;

fn main() {
    println!("Display the banner of the day !");

    println!("
███████╗██╗  ██╗████████╗██╗ █████╗
██╔════╝╚██╗██╔╝╚══██╔══╝██║██╔══██╗
█████╗   ╚███╔╝    ██║   ██║███████║
██╔══╝   ██╔██╗    ██║   ██║██╔══██║
███████╗██╔╝ ██╗   ██║   ██║██║  ██║
╚══════╝╚═╝  ╚═╝   ╚═╝   ╚═╝╚═╝  ╚═╝

███████╗ ██████╗ ██████╗     ██████╗ ██╗   ██╗███████╗████████╗
██╔════╝██╔═══██╗██╔══██╗    ██╔══██╗██║   ██║██╔════╝╚══██╔══╝
█████╗  ██║   ██║██████╔╝    ██████╔╝██║   ██║███████╗   ██║
██╔══╝  ██║   ██║██╔══██╗    ██╔══██╗██║   ██║╚════██║   ██║
██║     ╚██████╔╝██║  ██║    ██║  ██║╚██████╔╝███████║   ██║
╚═╝      ╚═════╝ ╚═╝  ╚═╝    ╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝ ");

    println!("F(x)? wich x you want");
    let mut nb_loop = String::new();
    io::stdin().read_line(&mut nb_loop)
        .expect("Failed to read! ");

    let nb_loop_int = nb_loop.trim().parse::<i32>().unwrap();

    println!("F({nb_loop_int}) = {}", fibo(nb_loop_int, 0, 1));
}


fn fibo(mut nb_loop: i32, left: i32, right: i32) -> i32
{
    if (nb_loop > 0)
    {
        nb_loop = nb_loop - 1;
        let sum = left + right;

        return fibo(nb_loop, right, sum);
    }
    left
    // or return left
}



