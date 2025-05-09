
vim.api.nvim_create_autocmd("ColorScheme", {
  pattern = "*",
  callback = function()
    vim.api.nvim_set_hl(0, "Normal", { bg = "none" })
    vim.api.nvim_set_hl(0, "NormalNC", { bg = "none" })
    vim.api.nvim_set_hl(0, "NormalFloat", { bg = "none" })
    vim.api.nvim_set_hl(0, "FloatBorder", { bg = "none" })
    vim.api.nvim_set_hl(0, "VertSplit", { bg = "none" })
  end,
})




return {
  {
    "ellisonleao/gruvbox.nvim",
    priority = 1000, -- чтобы тема загрузилась раньше всего
    opts = {
      transparent_mode = true,
    },
    config = function(_, opts)
      require("gruvbox").setup(opts)
      vim.cmd("colorscheme gruvbox")
    end,
  },
}




vim.api.nvim_create_autocmd("ColorScheme", {
  pattern = "*",
  callback = function()
    vim.cmd("hi Normal guibg=NONE ctermbg=NONE")
    vim.cmd("hi NormalNC guibg=NONE ctermbg=NONE")
    vim.cmd("hi NormalFloat guibg=NONE ctermbg=NONE")
    vim.cmd("hi FloatBorder guibg=NONE ctermbg=NONE")
    vim.cmd("hi VertSplit guibg=NONE ctermbg=NONE")
    vim.cmd("hi SignColumn guibg=NONE ctermbg=NONE")
    vim.cmd("hi StatusLine guibg=NONE ctermbg=NONE")
    vim.cmd("hi StatusLineNC guibg=NONE ctermbg=NONE")
    vim.cmd("hi Pmenu guibg=NONE ctermbg=NONE")
    vim.cmd("hi PmenuSel guibg=NONE ctermbg=NONE")
    vim.cmd("hi PmenuSbar guibg=NONE ctermbg=NONE")
    vim.cmd("hi PmenuThumb guibg=NONE ctermbg=NONE")
    vim.cmd("hi MsgArea guibg=NONE ctermbg=NONE")
    vim.cmd("hi CmdLine guibg=NONE ctermbg=NONE")
    vim.cmd("hi TelescopeNormal guibg=NONE ctermbg=NONE")
    vim.cmd("hi TelescopeBorder guibg=NONE ctermbg=NONE")
  end,
})



use std::thread;
use std::time::Duration;
use crossterm::{
    execute, queue,
    terminal::{Clear, ClearType},
    cursor::MoveTo,
    style::Print,
    event::{poll, read, Event, KeyCode},
};

fn main() -> std::io::Result<()> {
    let mut counter = 0;
    loop {
        if poll(Duration::from_millis(16))? {
            if let Event::Key(event) = read()? {
                if event.code == KeyCode::Esc {
                    break;
                }
            }
        }

        // Очищаем экран, перемещаем курсор и выводим счётчик
        counter += 1;
        queue!(
            std::io::stdout(),
            Clear(ClearType::All),          // Очистка экрана
            MoveTo(0, 0),                   // Курсор в верхний левый угол
            Print(format!("Counter: {}", counter))  // Выводим счётчик
        )?;

        // Выполняем все команды
        execute!(std::io::stdout())?;

        thread::sleep(Duration::from_secs(1));
    }

    // Очищаем экран перед выходом
    execute!(std::io::stdout(), Clear(ClearType::All))?;
    Ok(())
}


use std::thread;
use std::time::Duration;
use crossterm::{
    execute, queue,
    terminal::{Clear, ClearType, enable_raw_mode, disable_raw_mode},
    cursor::MoveTo,
    style::Print,
    event::{poll, read, Event, KeyCode},
};

fn main() -> std::io::Result<()> {
    // Включаем raw mode для корректного чтения клавиш
    enable_raw_mode()?;

    let mut counter = 0;
    loop {
        // Увеличиваем таймаут до 100 мс для надёжного чтения
        if poll(Duration::from_millis(100))? {
            let event = read()?;
            // Отладочный вывод: показываем, какое событие пришло
            println!("Event: {:?}", event);

            if let Event::Key(event) = event {
                // Проверяем, что за клавиша
                match event.code {
                    KeyCode::Esc => {
                        println!("Esc pressed, exiting...");
                        break;
                    }
                    KeyCode::Char('q') => {
                        println!("q pressed, exiting...");
                        break;
                    }
                    _ => println!("Pressed: {:?}", event.code),
                }
            }
        }

        counter += 1;
        queue!(
            std::io::stdout(),
            Clear(ClearType::All),
            MoveTo(0, 0),
            Print(format!("Counter: {}", counter))
        )?;
        execute!(std::io::stdout())?;

        thread::sleep(Duration::from_secs(1));
    }

    // Выключаем raw mode и очищаем экран перед выходом
    disable_raw_mode()?;
    execute!(std::io::stdout(), Clear(ClearType::All))?;
    Ok(())
}
