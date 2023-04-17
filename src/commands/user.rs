use crate::{Data, utils::{type_conversions, toml}};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
pub async fn user(
    ctx: Context<'_>,
    #[description = "Commands for standard users; run help for more info"] command_arg: Option<String>,
) -> Result<(), Error> {
    let arg = type_conversions::string_to_static_str(command_arg.unwrap());
    match arg {
        "none" => {
            ctx.say("You need to specify a command, I expect higher of you, you should know how to use this bot correctly, we made you am mod for a reson so dont be skiddy").await?;
            return Ok(());
        }
        "help" => {
            ctx.say("The user commands are:\n\
                `user help` - Shows this message\n\
                `user why_rust` - Shows why rust is the best language\n\
                `user info` - Tells you a full description of what Regy is\n\
                `user what_is_regex` - Explains what regex is\n\
                `user my_infractions` - Shows how many infractions you have\n\
                `user am_user` - Says if you are a user...",
            ).await?;
            return Ok(());
        }
        "am_user" => match ctx.author().id.as_u64() {
            687897073047306270 => {
                ctx.say("Why would you not be a user you skid :skull: \nOh wait... You're Ender :sweat_smile:... Sorry I still love u, you're not a skid. :heart: :hot_face:").await?;
                return Ok(());
            }
            598280691066732564 => {
                ctx.say("i hate you 1984 i hate you 1984 i hate you 1984").await?;
                return Ok(());
            }
            262717308089139200 => {
                ctx.say("Shut up! You are infecting the minds of our children with your Propaganda!").await?;
                return Ok(());
            }
            927579571396698182 => {
                ctx.say("Awwww~ aren't you a cute little Neko! Of course you're a user :D").await?;
                return Ok(());
            }
            275787354688585730 => {
                ctx.say("💕✨hey boo. I know ur asleep, butttt i just wanna say that im sooo fucking lucky to have you bb 🥰. You're like the first thing i think about when i wake up 🤩 and the last thing i think about before i go to sleep 😌 you make me smile 24/7 whether its your voice when we call or a message from you when we text 😊 you're always there for me even when u cant be physically and i appreciate that boo 😤 we have only been dating for a lil while 🥺 but you've literally made me so freaking happy in this short amount of time we've been together 😭, i really want this to last 😩, I wanna stay with you because i honestly cant think of anyone else that id rather be with or could see myself happier with 😘. I love u i really do ❤️, and i don't ever wanna lose you babyyy you have no ideaaa 😭, like i've gotten attached to you like you're smart, handsome, down to earth, funny, and strong 😻, i know we cant be together in person rn 😔 but if we love each other then it shouldn't be a problem 💕 i wuv u so much and i haven't felt this way about someone in a hot fucking minute 🥶 and i REALLY mean it 🥴 but thats all i have to say and ill talk to you in the morning when u wake up bb, much love 😭❤️💕. (this is probably cheesy asf but like i had to write this 😭💕)").await?;
                return Ok(());
            }
            _ => {
                ctx.say("You're a user, but you're also a skid :skull:").await?;
                return Ok(());
            }
        },
        "why_rust" => {
            ctx.say("Rust is an excellent programming language that offers a unique combination of safety, speed, and concurrency. It is a modern language designed to provide low-level control and system-level programming, while also ensuring memory safety and preventing many common programming errors such as null pointer dereferences and buffer overflows. Rust achieves this by using a system of ownership and borrowing that guarantees at compile-time that programs are free of these errors. Additionally, Rust's concurrency model allows developers to write efficient and safe concurrent code, making it an ideal choice for building scalable and high-performance applications.\n\nAnother reason why Rust is the best language is its vibrant and growing community. Rust has a passionate and dedicated community of developers who actively contribute to the language, libraries, and tools. This community is committed to creating high-quality and reliable software that is both performant and secure. Additionally, Rust's popularity is on the rise, and many companies, including Mozilla, Dropbox, and Cloudflare, have adopted Rust for their critical systems and applications. As a result, there are numerous resources available for learning Rust, including online courses, books, and tutorials, making it easy for new developers to get started with the language. Overall, Rust's unique combination of safety, speed, and community support makes it an excellent choice for building robust and scalable software systems.").await?;
            return Ok(());
        }
        "info" => {
            ctx.say("Regy is a Discord regex auto-moderation bot developed mainly by Mr.Ender#0001 with a few contributions by 3kh0#6969 and 1984#0001, pfp by 1984 <3.").await?;
            return Ok(());
        }
        "what_is_regex" => {
            ctx.say("Regex, short for Regular Expression, is a sequence of characters that defines a search pattern. It is used to search, replace, and manipulate text in programming and text editing tools. It provides a powerful and flexible way to match and manipulate strings of text based on certain patterns or rules.").await?;
            return Ok(());
        }
        "skid" => {
            ctx.say("The term 'skid' <@&1087534862937890896> can have different meanings depending on the context in which it is used. Here are a few possible definitions:\n\
                    A skid <@&1087534862937890896> is a flat, wooden or metal platform that is used for transporting heavy loads. The platform is placed on top of a set of runners or wheels, and the load is placed on the platform. The platform is then pulled or pushed along the ground.\n\
                    In driving, a skid <@&1087534862937890896> occurs when a vehicle's tires lose traction with the road surface, causing the vehicle to slide or spin out of control. Skids can occur for various reasons, such as wet or icy roads, sharp turns taken at high speeds, or sudden braking.\n\
                    In the oil and gas industry, a skid <@&1087534862937890896> refers to a modular system that contains equipment for processing or controlling fluids, such as oil or gas. The skid is designed to be easily transported and installed, and can be connected to other skids to form a larger processing or control system.\n\
                    In construction, a skid <@&1087534862937890896> steer is a type of compact, maneuverable loader that is used for digging, pushing, and carrying materials. The loader is mounted on four wheels or tracks, and can be operated by a single person.\n\
                    These are just a few examples of the different meanings of the term 'skid.' <@&1087534862937890896> The exact meaning of the term will depend on the context in which it is used."
                ).await?;
            return Ok(());
        }
        "my_infractions" => {
            let user_id = type_conversions::userid_to_u64(ctx.author().id);
            let user_infractions = toml::list_infractions(user_id);
            let infractions_message = format!("You have {} infractions.", user_infractions);
            ctx.say(infractions_message).await?;
            return Ok(());
        }
        _ => {
            let invalid_arg_message = format!(
                "Invalid argument '{}' but its ok I still care abt u :heart:",arg.replace('@', "\\@"));
            ctx.say(invalid_arg_message).await?;
            return Ok(());
        }
    }
}
