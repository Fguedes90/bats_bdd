# language:pt
Funcionalidade: Suporte a i18n em Português
  Testar que o parser suporta keywords em português
  Todos os keywords Gherkin devem funcionar em português

  Cenário: Dado, Quando, Então em português
    Dado que tenho um sistema funcional
    Quando executo um teste em português
    Então o teste deve passar

  Cenário: E (And) em português
    Dado que inicializei o sistema
    E configurei os parâmetros
    Quando executo a validação
    Então o resultado deve ser esperado

  Cenário: Mas (But) em português
    Dado que tenho um cenário
    Quando verifico as condições
    Então tudo deve funcionar
    Mas não deve haver erros

  @regressão
  Cenário: Tags em português com acento
    Dado que tenho uma tag em português
    Então a tag deve ser parseada corretamente

  Esquema do Cenário: Exemplos em português
    Dado que calculo <operacao> <a> e <b>
    Então o resultado deve ser <resultado>

    Exemplos:
      | operacao | a | b  | resultado |
      | soma     | 2 | 3  | 5         |
      | subtracao| 5 | 2  | 3         |
