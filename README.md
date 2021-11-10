# optical-waveguide

<!---Esses são exemplos. Veja https://shields.io para outras pessoas ou para personalizar este conjunto de escudos. Você pode querer incluir dependências, status do projeto e informações de licença aqui--->

> Guia de onda optico para fotônica integrada.

## 💻 Pré-requisitos

Antes de começar, verifique se você atendeu aos seguintes requisitos:
<!---Estes são apenas requisitos de exemplo. Adicionar, duplicar ou remover conforme necessário--->
* Um ambiente de desenvolvimento Rust. Aconselhamos instalar a partir do `<rustup>`. 
Mais informações https://rust-lang.github.io/rustup/installation/index.html
* `<python3 >= 3.9>`
* `<HDF5 >= 1.12.1>`
* `<pip >= 21.3.1>`
* `<maturin >= 0.11.5>`
* `<Qt5 >= 5.15>`

## 🚀 Instalando optical_waveguide

Para instalar o <nome_do_projeto>, siga estas etapas:

Linux:
```
python - venv .env
source .env/bin/activate
maturin develop
```

## ☕ Usando <optical-waveguide>

Para executar um exemplo bidimensional, siga estas etapas:

```
python examples/bidimensional.py
```

Adicione comandos de execução e exemplos que você acha que os usuários acharão úteis. Fornece uma referência de opções para pontos de bônus!

## 📫 Contribuindo para <nome_do_projeto>
<!---Se o seu README for longo ou se você tiver algum processo ou etapas específicas que deseja que os contribuidores sigam, considere a criação de um arquivo CONTRIBUTING.md separado--->
Para contribuir com <nome_do_projeto>, siga estas etapas:

1. Bifurque este repositório.
2. Crie um branch: `git checkout -b <nome_branch>`.
3. Faça suas alterações e confirme-as: `git commit -m '<mensagem_commit>'`
4. Envie para o branch original: `git push origin <nome_do_projeto> / <local>`
5. Crie a solicitação de pull.

Como alternativa, consulte a documentação do GitHub em [como criar uma solicitação pull](https://help.github.com/en/github/collaborating-with-issues-and-pull-requests/creating-a-pull-request).

### Ajustes e melhorias

O projeto ainda está em desenvolvimento e as próximas atualizações serão voltadas nas seguintes tarefas:

- [ ] tornar todos os maps do codigo em paralelo
- [ ] otimizar dephts_cartesian_product
- [ ] criar composição de cores
- [ ] criar composição de beams

Infezlimente o código ainda não esta com cobertura total de testes. Um breve apanhado de casos de testes necessários:

- [ ] testas todas as condições de panic
- [ ] feixe gaussiano em regiões fora da grid de simulação

## 🤝 Colaboradores

Agradecemos às seguintes pessoas que contribuíram para este projeto:

<table>
  <tr>
    <td align="center">
      <a href="#">
        <img src="https://avatars.githubusercontent.com/u/34797226?s=400&u=2505230870aaf025b6c3e6806981d4567b594592&v=4" width="100px;" alt="Foto do rennelou no GitHub"/><br>
        <sub>
          <b>Rennê Lou</b>
        </sub>
      </a>
    </td>
  </tr>
</table>


## 😄 Seja um dos contribuidores<br>

Quer fazer parte desse projeto? Clique [AQUI](CONTRIBUTING.md) e leia como contribuir.

## 📝 Licença

Esse projeto está sob licença. Veja o arquivo [LICENÇA](LICENSE.md) para mais detalhes.

[⬆ Voltar ao topo](#optical-waveguide)<br>
