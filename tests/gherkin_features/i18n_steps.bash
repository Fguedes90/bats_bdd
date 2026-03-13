#!/usr/bin/env bash
# Step definitions for i18n_portuguese.feature (Portuguese steps)

# Cenário: Dado, Quando, Então em português
step_dado_que_tenho_um_sistema_funcional() {
  export SISTEMA_STATUS="funcional"
}

step_quando_executo_um_teste_em_portugues() {
  export TESTE_EXECUTADO="true"
}

step_entao_o_teste_deve_passar() {
  if [[ "$SISTEMA_STATUS" != "funcional" ]]; then
    echo "Sistema deve estar funcional" >&2
    return 1
  fi
}

# Cenário: E (And) em português
step_dado_que_inicializei_o_sistema() {
  export SISTEMA_INICIALIZADO="true"
}

step_configurei_os_parametros() {
  export PARAMETROS_CONFIGURADOS="true"
}

step_quando_executo_a_validacao() {
  export VALIDACAO_EXECUTADA="true"
}

step_entao_o_resultado_deve_ser_esperado() {
  if [[ "$SISTEMA_INICIALIZADO" != "true" ]] || [[ "$PARAMETROS_CONFIGURADOS" != "true" ]]; then
    echo "Pré-condições não atendidas" >&2
    return 1
  fi
}

# Cenário: Mas (But) em português
step_dado_que_tenho_um_cenario() {
  export CENARIO_EXISTENTE="true"
}

step_quando_verifico_as_condicoes() {
  export CONDICOES_VERIFICADAS="true"
}

step_entao_tudo_deve_funcionar() {
  if [[ "$CENARIO_EXISTENTE" != "true" ]]; then
    echo "Cenário deve existir" >&2
    return 1
  fi
}

step_nao_deve_haver_erros() {
  if [[ -n "$ERRO_ENCONTRADO" ]]; then
    echo "Erro encontrado: $ERRO_ENCONTRADO" >&2
    return 1
  fi
}

# Cenário: Tags em português com acento
step_dado_que_tenho_uma_tag_em_portugues() {
  export TAG_PORTUGUES="true"
}

step_entao_a_tag_deve_ser_parseada_corretamente() {
  if [[ "$TAG_PORTUGUES" != "true" ]]; then
    echo "Tag em português não foi parseada" >&2
    return 1
  fi
}

# Esquema do Cenário: Exemplos em português
step_dado_que_calculo_soma_2_e_3() {
  export RESULTADO=$((2 + 3))
}

step_entao_o_resultado_deve_ser_5() {
  if [[ "$RESULTADO" -ne 5 ]]; then
    echo "Esperado 5, obtido $RESULTADO" >&2
    return 1
  fi
}

step_dado_que_calculo_subtracao_5_e_2() {
  export RESULTADO=$((5 - 2))
}

step_entao_o_resultado_deve_ser_3() {
  if [[ "$RESULTADO" -ne 3 ]]; then
    echo "Esperado 3, obtido $RESULTADO" >&2
    return 1
  fi
}
