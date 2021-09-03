use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    /// Grave tarefas no arquivo diário
    Add {
        /// O texto de descrição da tarefa
        #[structopt()]
        text: String,
    },
    /// Remova uma tarefa do arquivo diário por posição
    Done {
        #[structopt()]
        position: usize,
    },
    /// Listar todas as tarefas no arquivo diário   
    List,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "Rusty Journal", about = "Uma tesk list escrito em Rust")]

pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,

    ///Use um arquivo diário diferente
    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>,
}
