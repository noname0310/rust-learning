use std::io;

fn main()
{
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    for i in 1..= n * 2 -1
    {
        if i <= n
        {
            for _j in 0..=i-2
            {
                print!(" ");
            }
            for j in 1..=n
            {
                if i == 1 || i == n * 2 -1
                {
                    print!("*");
                }
                else
                {
                    if j == 1 || j == n
                    {
                        print!("*");
                    }
                    else
                    {
                        print!(" ");
                    }
                }
            }
            for _j in 0..(n - i) * 2 - 1
            {
                print!(" ");
            }
            for j in 1..=n
            {
                if i == 1 || i == n * 2 -1
                {
                    print!("*");
                }
                else
                {
                    if j == 1
                    {
                        if i != n
                        {
                            print!("*");
                        }
                    }
                    else if j == n
                    {
                        print!("*");
                    }
                    else
                    {
                        print!(" ");
                    }
                }
            }
        }
        else
        {
            for _j in 0..i - (i - n) * 2 - 1
            {
                print!(" ");
            }
            for j in 1..=n
            {
                if i == 1 || i == n * 2 -1
                {
                    print!("*");
                }
                else
                {
                    if j == 1 || j == n
                    {
                        print!("*");
                    }
                    else
                    {
                        print!(" ");
                    }
                }
            }
            for _j in 0..(i - n) * 2 - 1
            {
                print!(" ");
            }
            for j in 1..=n
            {
                if i == 1 || i == n * 2 -1
                {
                    print!("*");
                }
                else
                {
                    if j == 1 || j == n
                    {
                        print!("*");
                    }
                    else
                    {
                        print!(" ");
                    }
                }
            }
        }
        println!();
    }
}