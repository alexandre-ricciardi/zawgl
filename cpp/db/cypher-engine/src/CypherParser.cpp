
// Generated from Cypher.g4 by ANTLR 4.7.2


#include "CypherListener.h"

#include "CypherParser.h"


using namespace antlrcpp;
using namespace antlr4;

CypherParser::CypherParser(TokenStream *input) : Parser(input) {
  _interpreter = new atn::ParserATNSimulator(this, _atn, _decisionToDFA, _sharedContextCache);
}

CypherParser::~CypherParser() {
  delete _interpreter;
}

std::string CypherParser::getGrammarFileName() const {
  return "Cypher.g4";
}

const std::vector<std::string>& CypherParser::getRuleNames() const {
  return _ruleNames;
}

dfa::Vocabulary& CypherParser::getVocabulary() const {
  return _vocabulary;
}


//----------------- OC_CypherContext ------------------------------------------------------------------

CypherParser::OC_CypherContext::OC_CypherContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

CypherParser::OC_StatementContext* CypherParser::OC_CypherContext::oC_Statement() {
  return getRuleContext<CypherParser::OC_StatementContext>(0);
}

tree::TerminalNode* CypherParser::OC_CypherContext::EOF() {
  return getToken(CypherParser::EOF, 0);
}

std::vector<tree::TerminalNode *> CypherParser::OC_CypherContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_CypherContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}


size_t CypherParser::OC_CypherContext::getRuleIndex() const {
  return CypherParser::RuleOC_Cypher;
}

void CypherParser::OC_CypherContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_Cypher(this);
}

void CypherParser::OC_CypherContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_Cypher(this);
}

CypherParser::OC_CypherContext* CypherParser::oC_Cypher() {
  OC_CypherContext *_localctx = _tracker.createInstance<OC_CypherContext>(_ctx, getState());
  enterRule(_localctx, 0, CypherParser::RuleOC_Cypher);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(199);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (_la == CypherParser::SP) {
      setState(198);
      match(CypherParser::SP);
    }
    setState(201);
    oC_Statement();
    setState(206);
    _errHandler->sync(this);

    switch (getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 2, _ctx)) {
    case 1: {
      setState(203);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(202);
        match(CypherParser::SP);
      }
      setState(205);
      match(CypherParser::T__0);
      break;
    }

    }
    setState(209);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (_la == CypherParser::SP) {
      setState(208);
      match(CypherParser::SP);
    }
    setState(211);
    match(CypherParser::EOF);
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_StatementContext ------------------------------------------------------------------

CypherParser::OC_StatementContext::OC_StatementContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

CypherParser::OC_QueryContext* CypherParser::OC_StatementContext::oC_Query() {
  return getRuleContext<CypherParser::OC_QueryContext>(0);
}


size_t CypherParser::OC_StatementContext::getRuleIndex() const {
  return CypherParser::RuleOC_Statement;
}

void CypherParser::OC_StatementContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_Statement(this);
}

void CypherParser::OC_StatementContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_Statement(this);
}

CypherParser::OC_StatementContext* CypherParser::oC_Statement() {
  OC_StatementContext *_localctx = _tracker.createInstance<OC_StatementContext>(_ctx, getState());
  enterRule(_localctx, 2, CypherParser::RuleOC_Statement);

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(213);
    oC_Query();
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_QueryContext ------------------------------------------------------------------

CypherParser::OC_QueryContext::OC_QueryContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

CypherParser::OC_RegularQueryContext* CypherParser::OC_QueryContext::oC_RegularQuery() {
  return getRuleContext<CypherParser::OC_RegularQueryContext>(0);
}

CypherParser::OC_StandaloneCallContext* CypherParser::OC_QueryContext::oC_StandaloneCall() {
  return getRuleContext<CypherParser::OC_StandaloneCallContext>(0);
}


size_t CypherParser::OC_QueryContext::getRuleIndex() const {
  return CypherParser::RuleOC_Query;
}

void CypherParser::OC_QueryContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_Query(this);
}

void CypherParser::OC_QueryContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_Query(this);
}

CypherParser::OC_QueryContext* CypherParser::oC_Query() {
  OC_QueryContext *_localctx = _tracker.createInstance<OC_QueryContext>(_ctx, getState());
  enterRule(_localctx, 4, CypherParser::RuleOC_Query);

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    setState(217);
    _errHandler->sync(this);
    switch (getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 4, _ctx)) {
    case 1: {
      enterOuterAlt(_localctx, 1);
      setState(215);
      oC_RegularQuery();
      break;
    }

    case 2: {
      enterOuterAlt(_localctx, 2);
      setState(216);
      oC_StandaloneCall();
      break;
    }

    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_RegularQueryContext ------------------------------------------------------------------

CypherParser::OC_RegularQueryContext::OC_RegularQueryContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

CypherParser::OC_SingleQueryContext* CypherParser::OC_RegularQueryContext::oC_SingleQuery() {
  return getRuleContext<CypherParser::OC_SingleQueryContext>(0);
}

std::vector<CypherParser::OC_UnionContext *> CypherParser::OC_RegularQueryContext::oC_Union() {
  return getRuleContexts<CypherParser::OC_UnionContext>();
}

CypherParser::OC_UnionContext* CypherParser::OC_RegularQueryContext::oC_Union(size_t i) {
  return getRuleContext<CypherParser::OC_UnionContext>(i);
}

std::vector<tree::TerminalNode *> CypherParser::OC_RegularQueryContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_RegularQueryContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}


size_t CypherParser::OC_RegularQueryContext::getRuleIndex() const {
  return CypherParser::RuleOC_RegularQuery;
}

void CypherParser::OC_RegularQueryContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_RegularQuery(this);
}

void CypherParser::OC_RegularQueryContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_RegularQuery(this);
}

CypherParser::OC_RegularQueryContext* CypherParser::oC_RegularQuery() {
  OC_RegularQueryContext *_localctx = _tracker.createInstance<OC_RegularQueryContext>(_ctx, getState());
  enterRule(_localctx, 6, CypherParser::RuleOC_RegularQuery);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    size_t alt;
    enterOuterAlt(_localctx, 1);
    setState(219);
    oC_SingleQuery();
    setState(226);
    _errHandler->sync(this);
    alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 6, _ctx);
    while (alt != 2 && alt != atn::ATN::INVALID_ALT_NUMBER) {
      if (alt == 1) {
        setState(221);
        _errHandler->sync(this);

        _la = _input->LA(1);
        if (_la == CypherParser::SP) {
          setState(220);
          match(CypherParser::SP);
        }
        setState(223);
        oC_Union(); 
      }
      setState(228);
      _errHandler->sync(this);
      alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 6, _ctx);
    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_UnionContext ------------------------------------------------------------------

CypherParser::OC_UnionContext::OC_UnionContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

tree::TerminalNode* CypherParser::OC_UnionContext::UNION() {
  return getToken(CypherParser::UNION, 0);
}

std::vector<tree::TerminalNode *> CypherParser::OC_UnionContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_UnionContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}

tree::TerminalNode* CypherParser::OC_UnionContext::ALL() {
  return getToken(CypherParser::ALL, 0);
}

CypherParser::OC_SingleQueryContext* CypherParser::OC_UnionContext::oC_SingleQuery() {
  return getRuleContext<CypherParser::OC_SingleQueryContext>(0);
}


size_t CypherParser::OC_UnionContext::getRuleIndex() const {
  return CypherParser::RuleOC_Union;
}

void CypherParser::OC_UnionContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_Union(this);
}

void CypherParser::OC_UnionContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_Union(this);
}

CypherParser::OC_UnionContext* CypherParser::oC_Union() {
  OC_UnionContext *_localctx = _tracker.createInstance<OC_UnionContext>(_ctx, getState());
  enterRule(_localctx, 8, CypherParser::RuleOC_Union);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    setState(241);
    _errHandler->sync(this);
    switch (getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 9, _ctx)) {
    case 1: {
      enterOuterAlt(_localctx, 1);
      setState(229);
      match(CypherParser::UNION);
      setState(230);
      match(CypherParser::SP);
      setState(231);
      match(CypherParser::ALL);
      setState(233);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(232);
        match(CypherParser::SP);
      }
      setState(235);
      oC_SingleQuery();
      break;
    }

    case 2: {
      enterOuterAlt(_localctx, 2);
      setState(236);
      match(CypherParser::UNION);
      setState(238);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(237);
        match(CypherParser::SP);
      }
      setState(240);
      oC_SingleQuery();
      break;
    }

    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_SingleQueryContext ------------------------------------------------------------------

CypherParser::OC_SingleQueryContext::OC_SingleQueryContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

CypherParser::OC_SinglePartQueryContext* CypherParser::OC_SingleQueryContext::oC_SinglePartQuery() {
  return getRuleContext<CypherParser::OC_SinglePartQueryContext>(0);
}

CypherParser::OC_MultiPartQueryContext* CypherParser::OC_SingleQueryContext::oC_MultiPartQuery() {
  return getRuleContext<CypherParser::OC_MultiPartQueryContext>(0);
}


size_t CypherParser::OC_SingleQueryContext::getRuleIndex() const {
  return CypherParser::RuleOC_SingleQuery;
}

void CypherParser::OC_SingleQueryContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_SingleQuery(this);
}

void CypherParser::OC_SingleQueryContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_SingleQuery(this);
}

CypherParser::OC_SingleQueryContext* CypherParser::oC_SingleQuery() {
  OC_SingleQueryContext *_localctx = _tracker.createInstance<OC_SingleQueryContext>(_ctx, getState());
  enterRule(_localctx, 10, CypherParser::RuleOC_SingleQuery);

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    setState(245);
    _errHandler->sync(this);
    switch (getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 10, _ctx)) {
    case 1: {
      enterOuterAlt(_localctx, 1);
      setState(243);
      oC_SinglePartQuery();
      break;
    }

    case 2: {
      enterOuterAlt(_localctx, 2);
      setState(244);
      oC_MultiPartQuery();
      break;
    }

    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_SinglePartQueryContext ------------------------------------------------------------------

CypherParser::OC_SinglePartQueryContext::OC_SinglePartQueryContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

CypherParser::OC_ReturnContext* CypherParser::OC_SinglePartQueryContext::oC_Return() {
  return getRuleContext<CypherParser::OC_ReturnContext>(0);
}

std::vector<CypherParser::OC_ReadingClauseContext *> CypherParser::OC_SinglePartQueryContext::oC_ReadingClause() {
  return getRuleContexts<CypherParser::OC_ReadingClauseContext>();
}

CypherParser::OC_ReadingClauseContext* CypherParser::OC_SinglePartQueryContext::oC_ReadingClause(size_t i) {
  return getRuleContext<CypherParser::OC_ReadingClauseContext>(i);
}

std::vector<tree::TerminalNode *> CypherParser::OC_SinglePartQueryContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_SinglePartQueryContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}

std::vector<CypherParser::OC_UpdatingClauseContext *> CypherParser::OC_SinglePartQueryContext::oC_UpdatingClause() {
  return getRuleContexts<CypherParser::OC_UpdatingClauseContext>();
}

CypherParser::OC_UpdatingClauseContext* CypherParser::OC_SinglePartQueryContext::oC_UpdatingClause(size_t i) {
  return getRuleContext<CypherParser::OC_UpdatingClauseContext>(i);
}


size_t CypherParser::OC_SinglePartQueryContext::getRuleIndex() const {
  return CypherParser::RuleOC_SinglePartQuery;
}

void CypherParser::OC_SinglePartQueryContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_SinglePartQuery(this);
}

void CypherParser::OC_SinglePartQueryContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_SinglePartQuery(this);
}

CypherParser::OC_SinglePartQueryContext* CypherParser::oC_SinglePartQuery() {
  OC_SinglePartQueryContext *_localctx = _tracker.createInstance<OC_SinglePartQueryContext>(_ctx, getState());
  enterRule(_localctx, 12, CypherParser::RuleOC_SinglePartQuery);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    size_t alt;
    setState(282);
    _errHandler->sync(this);
    switch (getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 19, _ctx)) {
    case 1: {
      enterOuterAlt(_localctx, 1);
      setState(253);
      _errHandler->sync(this);
      _la = _input->LA(1);
      while ((((_la & ~ 0x3fULL) == 0) &&
        ((1ULL << _la) & ((1ULL << CypherParser::OPTIONAL)
        | (1ULL << CypherParser::MATCH)
        | (1ULL << CypherParser::UNWIND)
        | (1ULL << CypherParser::CALL))) != 0)) {
        setState(247);
        oC_ReadingClause();
        setState(249);
        _errHandler->sync(this);

        _la = _input->LA(1);
        if (_la == CypherParser::SP) {
          setState(248);
          match(CypherParser::SP);
        }
        setState(255);
        _errHandler->sync(this);
        _la = _input->LA(1);
      }
      setState(256);
      oC_Return();
      break;
    }

    case 2: {
      enterOuterAlt(_localctx, 2);
      setState(263);
      _errHandler->sync(this);
      _la = _input->LA(1);
      while ((((_la & ~ 0x3fULL) == 0) &&
        ((1ULL << _la) & ((1ULL << CypherParser::OPTIONAL)
        | (1ULL << CypherParser::MATCH)
        | (1ULL << CypherParser::UNWIND)
        | (1ULL << CypherParser::CALL))) != 0)) {
        setState(257);
        oC_ReadingClause();
        setState(259);
        _errHandler->sync(this);

        _la = _input->LA(1);
        if (_la == CypherParser::SP) {
          setState(258);
          match(CypherParser::SP);
        }
        setState(265);
        _errHandler->sync(this);
        _la = _input->LA(1);
      }
      setState(266);
      oC_UpdatingClause();
      setState(273);
      _errHandler->sync(this);
      alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 16, _ctx);
      while (alt != 2 && alt != atn::ATN::INVALID_ALT_NUMBER) {
        if (alt == 1) {
          setState(268);
          _errHandler->sync(this);

          _la = _input->LA(1);
          if (_la == CypherParser::SP) {
            setState(267);
            match(CypherParser::SP);
          }
          setState(270);
          oC_UpdatingClause(); 
        }
        setState(275);
        _errHandler->sync(this);
        alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 16, _ctx);
      }
      setState(280);
      _errHandler->sync(this);

      switch (getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 18, _ctx)) {
      case 1: {
        setState(277);
        _errHandler->sync(this);

        _la = _input->LA(1);
        if (_la == CypherParser::SP) {
          setState(276);
          match(CypherParser::SP);
        }
        setState(279);
        oC_Return();
        break;
      }

      }
      break;
    }

    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_MultiPartQueryContext ------------------------------------------------------------------

CypherParser::OC_MultiPartQueryContext::OC_MultiPartQueryContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

CypherParser::OC_SinglePartQueryContext* CypherParser::OC_MultiPartQueryContext::oC_SinglePartQuery() {
  return getRuleContext<CypherParser::OC_SinglePartQueryContext>(0);
}

std::vector<CypherParser::OC_WithContext *> CypherParser::OC_MultiPartQueryContext::oC_With() {
  return getRuleContexts<CypherParser::OC_WithContext>();
}

CypherParser::OC_WithContext* CypherParser::OC_MultiPartQueryContext::oC_With(size_t i) {
  return getRuleContext<CypherParser::OC_WithContext>(i);
}

std::vector<CypherParser::OC_ReadingClauseContext *> CypherParser::OC_MultiPartQueryContext::oC_ReadingClause() {
  return getRuleContexts<CypherParser::OC_ReadingClauseContext>();
}

CypherParser::OC_ReadingClauseContext* CypherParser::OC_MultiPartQueryContext::oC_ReadingClause(size_t i) {
  return getRuleContext<CypherParser::OC_ReadingClauseContext>(i);
}

std::vector<CypherParser::OC_UpdatingClauseContext *> CypherParser::OC_MultiPartQueryContext::oC_UpdatingClause() {
  return getRuleContexts<CypherParser::OC_UpdatingClauseContext>();
}

CypherParser::OC_UpdatingClauseContext* CypherParser::OC_MultiPartQueryContext::oC_UpdatingClause(size_t i) {
  return getRuleContext<CypherParser::OC_UpdatingClauseContext>(i);
}

std::vector<tree::TerminalNode *> CypherParser::OC_MultiPartQueryContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_MultiPartQueryContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}


size_t CypherParser::OC_MultiPartQueryContext::getRuleIndex() const {
  return CypherParser::RuleOC_MultiPartQuery;
}

void CypherParser::OC_MultiPartQueryContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_MultiPartQuery(this);
}

void CypherParser::OC_MultiPartQueryContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_MultiPartQuery(this);
}

CypherParser::OC_MultiPartQueryContext* CypherParser::oC_MultiPartQuery() {
  OC_MultiPartQueryContext *_localctx = _tracker.createInstance<OC_MultiPartQueryContext>(_ctx, getState());
  enterRule(_localctx, 14, CypherParser::RuleOC_MultiPartQuery);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    size_t alt;
    enterOuterAlt(_localctx, 1);
    setState(306); 
    _errHandler->sync(this);
    alt = 1;
    do {
      switch (alt) {
        case 1: {
              setState(290);
              _errHandler->sync(this);
              _la = _input->LA(1);
              while ((((_la & ~ 0x3fULL) == 0) &&
                ((1ULL << _la) & ((1ULL << CypherParser::OPTIONAL)
                | (1ULL << CypherParser::MATCH)
                | (1ULL << CypherParser::UNWIND)
                | (1ULL << CypherParser::CALL))) != 0)) {
                setState(284);
                oC_ReadingClause();
                setState(286);
                _errHandler->sync(this);

                _la = _input->LA(1);
                if (_la == CypherParser::SP) {
                  setState(285);
                  match(CypherParser::SP);
                }
                setState(292);
                _errHandler->sync(this);
                _la = _input->LA(1);
              }
              setState(299);
              _errHandler->sync(this);
              _la = _input->LA(1);
              while ((((_la & ~ 0x3fULL) == 0) &&
                ((1ULL << _la) & ((1ULL << CypherParser::MERGE)
                | (1ULL << CypherParser::CREATE)
                | (1ULL << CypherParser::SET)
                | (1ULL << CypherParser::DETACH)
                | (1ULL << CypherParser::DELETE)
                | (1ULL << CypherParser::REMOVE))) != 0)) {
                setState(293);
                oC_UpdatingClause();
                setState(295);
                _errHandler->sync(this);

                _la = _input->LA(1);
                if (_la == CypherParser::SP) {
                  setState(294);
                  match(CypherParser::SP);
                }
                setState(301);
                _errHandler->sync(this);
                _la = _input->LA(1);
              }
              setState(302);
              oC_With();
              setState(304);
              _errHandler->sync(this);

              _la = _input->LA(1);
              if (_la == CypherParser::SP) {
                setState(303);
                match(CypherParser::SP);
              }
              break;
            }

      default:
        throw NoViableAltException(this);
      }
      setState(308); 
      _errHandler->sync(this);
      alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 25, _ctx);
    } while (alt != 2 && alt != atn::ATN::INVALID_ALT_NUMBER);
    setState(310);
    oC_SinglePartQuery();
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_UpdatingClauseContext ------------------------------------------------------------------

CypherParser::OC_UpdatingClauseContext::OC_UpdatingClauseContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

CypherParser::OC_CreateContext* CypherParser::OC_UpdatingClauseContext::oC_Create() {
  return getRuleContext<CypherParser::OC_CreateContext>(0);
}

CypherParser::OC_MergeContext* CypherParser::OC_UpdatingClauseContext::oC_Merge() {
  return getRuleContext<CypherParser::OC_MergeContext>(0);
}

CypherParser::OC_DeleteContext* CypherParser::OC_UpdatingClauseContext::oC_Delete() {
  return getRuleContext<CypherParser::OC_DeleteContext>(0);
}

CypherParser::OC_SetContext* CypherParser::OC_UpdatingClauseContext::oC_Set() {
  return getRuleContext<CypherParser::OC_SetContext>(0);
}

CypherParser::OC_RemoveContext* CypherParser::OC_UpdatingClauseContext::oC_Remove() {
  return getRuleContext<CypherParser::OC_RemoveContext>(0);
}


size_t CypherParser::OC_UpdatingClauseContext::getRuleIndex() const {
  return CypherParser::RuleOC_UpdatingClause;
}

void CypherParser::OC_UpdatingClauseContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_UpdatingClause(this);
}

void CypherParser::OC_UpdatingClauseContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_UpdatingClause(this);
}

CypherParser::OC_UpdatingClauseContext* CypherParser::oC_UpdatingClause() {
  OC_UpdatingClauseContext *_localctx = _tracker.createInstance<OC_UpdatingClauseContext>(_ctx, getState());
  enterRule(_localctx, 16, CypherParser::RuleOC_UpdatingClause);

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    setState(317);
    _errHandler->sync(this);
    switch (_input->LA(1)) {
      case CypherParser::CREATE: {
        enterOuterAlt(_localctx, 1);
        setState(312);
        oC_Create();
        break;
      }

      case CypherParser::MERGE: {
        enterOuterAlt(_localctx, 2);
        setState(313);
        oC_Merge();
        break;
      }

      case CypherParser::DETACH:
      case CypherParser::DELETE: {
        enterOuterAlt(_localctx, 3);
        setState(314);
        oC_Delete();
        break;
      }

      case CypherParser::SET: {
        enterOuterAlt(_localctx, 4);
        setState(315);
        oC_Set();
        break;
      }

      case CypherParser::REMOVE: {
        enterOuterAlt(_localctx, 5);
        setState(316);
        oC_Remove();
        break;
      }

    default:
      throw NoViableAltException(this);
    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_ReadingClauseContext ------------------------------------------------------------------

CypherParser::OC_ReadingClauseContext::OC_ReadingClauseContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

CypherParser::OC_MatchContext* CypherParser::OC_ReadingClauseContext::oC_Match() {
  return getRuleContext<CypherParser::OC_MatchContext>(0);
}

CypherParser::OC_UnwindContext* CypherParser::OC_ReadingClauseContext::oC_Unwind() {
  return getRuleContext<CypherParser::OC_UnwindContext>(0);
}

CypherParser::OC_InQueryCallContext* CypherParser::OC_ReadingClauseContext::oC_InQueryCall() {
  return getRuleContext<CypherParser::OC_InQueryCallContext>(0);
}


size_t CypherParser::OC_ReadingClauseContext::getRuleIndex() const {
  return CypherParser::RuleOC_ReadingClause;
}

void CypherParser::OC_ReadingClauseContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_ReadingClause(this);
}

void CypherParser::OC_ReadingClauseContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_ReadingClause(this);
}

CypherParser::OC_ReadingClauseContext* CypherParser::oC_ReadingClause() {
  OC_ReadingClauseContext *_localctx = _tracker.createInstance<OC_ReadingClauseContext>(_ctx, getState());
  enterRule(_localctx, 18, CypherParser::RuleOC_ReadingClause);

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    setState(322);
    _errHandler->sync(this);
    switch (_input->LA(1)) {
      case CypherParser::OPTIONAL:
      case CypherParser::MATCH: {
        enterOuterAlt(_localctx, 1);
        setState(319);
        oC_Match();
        break;
      }

      case CypherParser::UNWIND: {
        enterOuterAlt(_localctx, 2);
        setState(320);
        oC_Unwind();
        break;
      }

      case CypherParser::CALL: {
        enterOuterAlt(_localctx, 3);
        setState(321);
        oC_InQueryCall();
        break;
      }

    default:
      throw NoViableAltException(this);
    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_MatchContext ------------------------------------------------------------------

CypherParser::OC_MatchContext::OC_MatchContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

tree::TerminalNode* CypherParser::OC_MatchContext::MATCH() {
  return getToken(CypherParser::MATCH, 0);
}

CypherParser::OC_PatternContext* CypherParser::OC_MatchContext::oC_Pattern() {
  return getRuleContext<CypherParser::OC_PatternContext>(0);
}

tree::TerminalNode* CypherParser::OC_MatchContext::OPTIONAL() {
  return getToken(CypherParser::OPTIONAL, 0);
}

std::vector<tree::TerminalNode *> CypherParser::OC_MatchContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_MatchContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}

CypherParser::OC_WhereContext* CypherParser::OC_MatchContext::oC_Where() {
  return getRuleContext<CypherParser::OC_WhereContext>(0);
}


size_t CypherParser::OC_MatchContext::getRuleIndex() const {
  return CypherParser::RuleOC_Match;
}

void CypherParser::OC_MatchContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_Match(this);
}

void CypherParser::OC_MatchContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_Match(this);
}

CypherParser::OC_MatchContext* CypherParser::oC_Match() {
  OC_MatchContext *_localctx = _tracker.createInstance<OC_MatchContext>(_ctx, getState());
  enterRule(_localctx, 20, CypherParser::RuleOC_Match);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(326);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (_la == CypherParser::OPTIONAL) {
      setState(324);
      match(CypherParser::OPTIONAL);
      setState(325);
      match(CypherParser::SP);
    }
    setState(328);
    match(CypherParser::MATCH);
    setState(330);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (_la == CypherParser::SP) {
      setState(329);
      match(CypherParser::SP);
    }
    setState(332);
    oC_Pattern();
    setState(337);
    _errHandler->sync(this);

    switch (getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 31, _ctx)) {
    case 1: {
      setState(334);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(333);
        match(CypherParser::SP);
      }
      setState(336);
      oC_Where();
      break;
    }

    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_UnwindContext ------------------------------------------------------------------

CypherParser::OC_UnwindContext::OC_UnwindContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

tree::TerminalNode* CypherParser::OC_UnwindContext::UNWIND() {
  return getToken(CypherParser::UNWIND, 0);
}

CypherParser::OC_ExpressionContext* CypherParser::OC_UnwindContext::oC_Expression() {
  return getRuleContext<CypherParser::OC_ExpressionContext>(0);
}

std::vector<tree::TerminalNode *> CypherParser::OC_UnwindContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_UnwindContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}

tree::TerminalNode* CypherParser::OC_UnwindContext::AS() {
  return getToken(CypherParser::AS, 0);
}

CypherParser::OC_VariableContext* CypherParser::OC_UnwindContext::oC_Variable() {
  return getRuleContext<CypherParser::OC_VariableContext>(0);
}


size_t CypherParser::OC_UnwindContext::getRuleIndex() const {
  return CypherParser::RuleOC_Unwind;
}

void CypherParser::OC_UnwindContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_Unwind(this);
}

void CypherParser::OC_UnwindContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_Unwind(this);
}

CypherParser::OC_UnwindContext* CypherParser::oC_Unwind() {
  OC_UnwindContext *_localctx = _tracker.createInstance<OC_UnwindContext>(_ctx, getState());
  enterRule(_localctx, 22, CypherParser::RuleOC_Unwind);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(339);
    match(CypherParser::UNWIND);
    setState(341);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (_la == CypherParser::SP) {
      setState(340);
      match(CypherParser::SP);
    }
    setState(343);
    oC_Expression();
    setState(344);
    match(CypherParser::SP);
    setState(345);
    match(CypherParser::AS);
    setState(346);
    match(CypherParser::SP);
    setState(347);
    oC_Variable();
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_MergeContext ------------------------------------------------------------------

CypherParser::OC_MergeContext::OC_MergeContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

tree::TerminalNode* CypherParser::OC_MergeContext::MERGE() {
  return getToken(CypherParser::MERGE, 0);
}

CypherParser::OC_PatternPartContext* CypherParser::OC_MergeContext::oC_PatternPart() {
  return getRuleContext<CypherParser::OC_PatternPartContext>(0);
}

std::vector<tree::TerminalNode *> CypherParser::OC_MergeContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_MergeContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}

std::vector<CypherParser::OC_MergeActionContext *> CypherParser::OC_MergeContext::oC_MergeAction() {
  return getRuleContexts<CypherParser::OC_MergeActionContext>();
}

CypherParser::OC_MergeActionContext* CypherParser::OC_MergeContext::oC_MergeAction(size_t i) {
  return getRuleContext<CypherParser::OC_MergeActionContext>(i);
}


size_t CypherParser::OC_MergeContext::getRuleIndex() const {
  return CypherParser::RuleOC_Merge;
}

void CypherParser::OC_MergeContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_Merge(this);
}

void CypherParser::OC_MergeContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_Merge(this);
}

CypherParser::OC_MergeContext* CypherParser::oC_Merge() {
  OC_MergeContext *_localctx = _tracker.createInstance<OC_MergeContext>(_ctx, getState());
  enterRule(_localctx, 24, CypherParser::RuleOC_Merge);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    size_t alt;
    enterOuterAlt(_localctx, 1);
    setState(349);
    match(CypherParser::MERGE);
    setState(351);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (_la == CypherParser::SP) {
      setState(350);
      match(CypherParser::SP);
    }
    setState(353);
    oC_PatternPart();
    setState(358);
    _errHandler->sync(this);
    alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 34, _ctx);
    while (alt != 2 && alt != atn::ATN::INVALID_ALT_NUMBER) {
      if (alt == 1) {
        setState(354);
        match(CypherParser::SP);
        setState(355);
        oC_MergeAction(); 
      }
      setState(360);
      _errHandler->sync(this);
      alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 34, _ctx);
    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_MergeActionContext ------------------------------------------------------------------

CypherParser::OC_MergeActionContext::OC_MergeActionContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

tree::TerminalNode* CypherParser::OC_MergeActionContext::ON() {
  return getToken(CypherParser::ON, 0);
}

std::vector<tree::TerminalNode *> CypherParser::OC_MergeActionContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_MergeActionContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}

tree::TerminalNode* CypherParser::OC_MergeActionContext::MATCH() {
  return getToken(CypherParser::MATCH, 0);
}

CypherParser::OC_SetContext* CypherParser::OC_MergeActionContext::oC_Set() {
  return getRuleContext<CypherParser::OC_SetContext>(0);
}

tree::TerminalNode* CypherParser::OC_MergeActionContext::CREATE() {
  return getToken(CypherParser::CREATE, 0);
}


size_t CypherParser::OC_MergeActionContext::getRuleIndex() const {
  return CypherParser::RuleOC_MergeAction;
}

void CypherParser::OC_MergeActionContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_MergeAction(this);
}

void CypherParser::OC_MergeActionContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_MergeAction(this);
}

CypherParser::OC_MergeActionContext* CypherParser::oC_MergeAction() {
  OC_MergeActionContext *_localctx = _tracker.createInstance<OC_MergeActionContext>(_ctx, getState());
  enterRule(_localctx, 26, CypherParser::RuleOC_MergeAction);

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    setState(371);
    _errHandler->sync(this);
    switch (getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 35, _ctx)) {
    case 1: {
      enterOuterAlt(_localctx, 1);
      setState(361);
      match(CypherParser::ON);
      setState(362);
      match(CypherParser::SP);
      setState(363);
      match(CypherParser::MATCH);
      setState(364);
      match(CypherParser::SP);
      setState(365);
      oC_Set();
      break;
    }

    case 2: {
      enterOuterAlt(_localctx, 2);
      setState(366);
      match(CypherParser::ON);
      setState(367);
      match(CypherParser::SP);
      setState(368);
      match(CypherParser::CREATE);
      setState(369);
      match(CypherParser::SP);
      setState(370);
      oC_Set();
      break;
    }

    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_CreateContext ------------------------------------------------------------------

CypherParser::OC_CreateContext::OC_CreateContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

tree::TerminalNode* CypherParser::OC_CreateContext::CREATE() {
  return getToken(CypherParser::CREATE, 0);
}

CypherParser::OC_PatternContext* CypherParser::OC_CreateContext::oC_Pattern() {
  return getRuleContext<CypherParser::OC_PatternContext>(0);
}

tree::TerminalNode* CypherParser::OC_CreateContext::SP() {
  return getToken(CypherParser::SP, 0);
}


size_t CypherParser::OC_CreateContext::getRuleIndex() const {
  return CypherParser::RuleOC_Create;
}

void CypherParser::OC_CreateContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_Create(this);
}

void CypherParser::OC_CreateContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_Create(this);
}

CypherParser::OC_CreateContext* CypherParser::oC_Create() {
  OC_CreateContext *_localctx = _tracker.createInstance<OC_CreateContext>(_ctx, getState());
  enterRule(_localctx, 28, CypherParser::RuleOC_Create);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(373);
    match(CypherParser::CREATE);
    setState(375);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (_la == CypherParser::SP) {
      setState(374);
      match(CypherParser::SP);
    }
    setState(377);
    oC_Pattern();
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_SetContext ------------------------------------------------------------------

CypherParser::OC_SetContext::OC_SetContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

tree::TerminalNode* CypherParser::OC_SetContext::SET() {
  return getToken(CypherParser::SET, 0);
}

std::vector<CypherParser::OC_SetItemContext *> CypherParser::OC_SetContext::oC_SetItem() {
  return getRuleContexts<CypherParser::OC_SetItemContext>();
}

CypherParser::OC_SetItemContext* CypherParser::OC_SetContext::oC_SetItem(size_t i) {
  return getRuleContext<CypherParser::OC_SetItemContext>(i);
}

tree::TerminalNode* CypherParser::OC_SetContext::SP() {
  return getToken(CypherParser::SP, 0);
}


size_t CypherParser::OC_SetContext::getRuleIndex() const {
  return CypherParser::RuleOC_Set;
}

void CypherParser::OC_SetContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_Set(this);
}

void CypherParser::OC_SetContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_Set(this);
}

CypherParser::OC_SetContext* CypherParser::oC_Set() {
  OC_SetContext *_localctx = _tracker.createInstance<OC_SetContext>(_ctx, getState());
  enterRule(_localctx, 30, CypherParser::RuleOC_Set);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(379);
    match(CypherParser::SET);
    setState(381);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (_la == CypherParser::SP) {
      setState(380);
      match(CypherParser::SP);
    }
    setState(383);
    oC_SetItem();
    setState(388);
    _errHandler->sync(this);
    _la = _input->LA(1);
    while (_la == CypherParser::T__1) {
      setState(384);
      match(CypherParser::T__1);
      setState(385);
      oC_SetItem();
      setState(390);
      _errHandler->sync(this);
      _la = _input->LA(1);
    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_SetItemContext ------------------------------------------------------------------

CypherParser::OC_SetItemContext::OC_SetItemContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

CypherParser::OC_PropertyExpressionContext* CypherParser::OC_SetItemContext::oC_PropertyExpression() {
  return getRuleContext<CypherParser::OC_PropertyExpressionContext>(0);
}

CypherParser::OC_ExpressionContext* CypherParser::OC_SetItemContext::oC_Expression() {
  return getRuleContext<CypherParser::OC_ExpressionContext>(0);
}

std::vector<tree::TerminalNode *> CypherParser::OC_SetItemContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_SetItemContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}

CypherParser::OC_VariableContext* CypherParser::OC_SetItemContext::oC_Variable() {
  return getRuleContext<CypherParser::OC_VariableContext>(0);
}

CypherParser::OC_NodeLabelsContext* CypherParser::OC_SetItemContext::oC_NodeLabels() {
  return getRuleContext<CypherParser::OC_NodeLabelsContext>(0);
}


size_t CypherParser::OC_SetItemContext::getRuleIndex() const {
  return CypherParser::RuleOC_SetItem;
}

void CypherParser::OC_SetItemContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_SetItem(this);
}

void CypherParser::OC_SetItemContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_SetItem(this);
}

CypherParser::OC_SetItemContext* CypherParser::oC_SetItem() {
  OC_SetItemContext *_localctx = _tracker.createInstance<OC_SetItemContext>(_ctx, getState());
  enterRule(_localctx, 32, CypherParser::RuleOC_SetItem);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    setState(427);
    _errHandler->sync(this);
    switch (getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 46, _ctx)) {
    case 1: {
      enterOuterAlt(_localctx, 1);
      setState(391);
      oC_PropertyExpression();
      setState(393);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(392);
        match(CypherParser::SP);
      }
      setState(395);
      match(CypherParser::T__2);
      setState(397);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(396);
        match(CypherParser::SP);
      }
      setState(399);
      oC_Expression();
      break;
    }

    case 2: {
      enterOuterAlt(_localctx, 2);
      setState(401);
      oC_Variable();
      setState(403);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(402);
        match(CypherParser::SP);
      }
      setState(405);
      match(CypherParser::T__2);
      setState(407);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(406);
        match(CypherParser::SP);
      }
      setState(409);
      oC_Expression();
      break;
    }

    case 3: {
      enterOuterAlt(_localctx, 3);
      setState(411);
      oC_Variable();
      setState(413);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(412);
        match(CypherParser::SP);
      }
      setState(415);
      match(CypherParser::T__3);
      setState(417);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(416);
        match(CypherParser::SP);
      }
      setState(419);
      oC_Expression();
      break;
    }

    case 4: {
      enterOuterAlt(_localctx, 4);
      setState(421);
      oC_Variable();
      setState(423);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(422);
        match(CypherParser::SP);
      }
      setState(425);
      oC_NodeLabels();
      break;
    }

    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_DeleteContext ------------------------------------------------------------------

CypherParser::OC_DeleteContext::OC_DeleteContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

tree::TerminalNode* CypherParser::OC_DeleteContext::DELETE() {
  return getToken(CypherParser::DELETE, 0);
}

std::vector<CypherParser::OC_ExpressionContext *> CypherParser::OC_DeleteContext::oC_Expression() {
  return getRuleContexts<CypherParser::OC_ExpressionContext>();
}

CypherParser::OC_ExpressionContext* CypherParser::OC_DeleteContext::oC_Expression(size_t i) {
  return getRuleContext<CypherParser::OC_ExpressionContext>(i);
}

tree::TerminalNode* CypherParser::OC_DeleteContext::DETACH() {
  return getToken(CypherParser::DETACH, 0);
}

std::vector<tree::TerminalNode *> CypherParser::OC_DeleteContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_DeleteContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}


size_t CypherParser::OC_DeleteContext::getRuleIndex() const {
  return CypherParser::RuleOC_Delete;
}

void CypherParser::OC_DeleteContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_Delete(this);
}

void CypherParser::OC_DeleteContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_Delete(this);
}

CypherParser::OC_DeleteContext* CypherParser::oC_Delete() {
  OC_DeleteContext *_localctx = _tracker.createInstance<OC_DeleteContext>(_ctx, getState());
  enterRule(_localctx, 34, CypherParser::RuleOC_Delete);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    size_t alt;
    enterOuterAlt(_localctx, 1);
    setState(431);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (_la == CypherParser::DETACH) {
      setState(429);
      match(CypherParser::DETACH);
      setState(430);
      match(CypherParser::SP);
    }
    setState(433);
    match(CypherParser::DELETE);
    setState(435);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (_la == CypherParser::SP) {
      setState(434);
      match(CypherParser::SP);
    }
    setState(437);
    oC_Expression();
    setState(448);
    _errHandler->sync(this);
    alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 51, _ctx);
    while (alt != 2 && alt != atn::ATN::INVALID_ALT_NUMBER) {
      if (alt == 1) {
        setState(439);
        _errHandler->sync(this);

        _la = _input->LA(1);
        if (_la == CypherParser::SP) {
          setState(438);
          match(CypherParser::SP);
        }
        setState(441);
        match(CypherParser::T__1);
        setState(443);
        _errHandler->sync(this);

        _la = _input->LA(1);
        if (_la == CypherParser::SP) {
          setState(442);
          match(CypherParser::SP);
        }
        setState(445);
        oC_Expression(); 
      }
      setState(450);
      _errHandler->sync(this);
      alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 51, _ctx);
    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_RemoveContext ------------------------------------------------------------------

CypherParser::OC_RemoveContext::OC_RemoveContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

tree::TerminalNode* CypherParser::OC_RemoveContext::REMOVE() {
  return getToken(CypherParser::REMOVE, 0);
}

std::vector<tree::TerminalNode *> CypherParser::OC_RemoveContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_RemoveContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}

std::vector<CypherParser::OC_RemoveItemContext *> CypherParser::OC_RemoveContext::oC_RemoveItem() {
  return getRuleContexts<CypherParser::OC_RemoveItemContext>();
}

CypherParser::OC_RemoveItemContext* CypherParser::OC_RemoveContext::oC_RemoveItem(size_t i) {
  return getRuleContext<CypherParser::OC_RemoveItemContext>(i);
}


size_t CypherParser::OC_RemoveContext::getRuleIndex() const {
  return CypherParser::RuleOC_Remove;
}

void CypherParser::OC_RemoveContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_Remove(this);
}

void CypherParser::OC_RemoveContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_Remove(this);
}

CypherParser::OC_RemoveContext* CypherParser::oC_Remove() {
  OC_RemoveContext *_localctx = _tracker.createInstance<OC_RemoveContext>(_ctx, getState());
  enterRule(_localctx, 36, CypherParser::RuleOC_Remove);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    size_t alt;
    enterOuterAlt(_localctx, 1);
    setState(451);
    match(CypherParser::REMOVE);
    setState(452);
    match(CypherParser::SP);
    setState(453);
    oC_RemoveItem();
    setState(464);
    _errHandler->sync(this);
    alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 54, _ctx);
    while (alt != 2 && alt != atn::ATN::INVALID_ALT_NUMBER) {
      if (alt == 1) {
        setState(455);
        _errHandler->sync(this);

        _la = _input->LA(1);
        if (_la == CypherParser::SP) {
          setState(454);
          match(CypherParser::SP);
        }
        setState(457);
        match(CypherParser::T__1);
        setState(459);
        _errHandler->sync(this);

        _la = _input->LA(1);
        if (_la == CypherParser::SP) {
          setState(458);
          match(CypherParser::SP);
        }
        setState(461);
        oC_RemoveItem(); 
      }
      setState(466);
      _errHandler->sync(this);
      alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 54, _ctx);
    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_RemoveItemContext ------------------------------------------------------------------

CypherParser::OC_RemoveItemContext::OC_RemoveItemContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

CypherParser::OC_VariableContext* CypherParser::OC_RemoveItemContext::oC_Variable() {
  return getRuleContext<CypherParser::OC_VariableContext>(0);
}

CypherParser::OC_NodeLabelsContext* CypherParser::OC_RemoveItemContext::oC_NodeLabels() {
  return getRuleContext<CypherParser::OC_NodeLabelsContext>(0);
}

CypherParser::OC_PropertyExpressionContext* CypherParser::OC_RemoveItemContext::oC_PropertyExpression() {
  return getRuleContext<CypherParser::OC_PropertyExpressionContext>(0);
}


size_t CypherParser::OC_RemoveItemContext::getRuleIndex() const {
  return CypherParser::RuleOC_RemoveItem;
}

void CypherParser::OC_RemoveItemContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_RemoveItem(this);
}

void CypherParser::OC_RemoveItemContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_RemoveItem(this);
}

CypherParser::OC_RemoveItemContext* CypherParser::oC_RemoveItem() {
  OC_RemoveItemContext *_localctx = _tracker.createInstance<OC_RemoveItemContext>(_ctx, getState());
  enterRule(_localctx, 38, CypherParser::RuleOC_RemoveItem);

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    setState(471);
    _errHandler->sync(this);
    switch (getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 55, _ctx)) {
    case 1: {
      enterOuterAlt(_localctx, 1);
      setState(467);
      oC_Variable();
      setState(468);
      oC_NodeLabels();
      break;
    }

    case 2: {
      enterOuterAlt(_localctx, 2);
      setState(470);
      oC_PropertyExpression();
      break;
    }

    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_InQueryCallContext ------------------------------------------------------------------

CypherParser::OC_InQueryCallContext::OC_InQueryCallContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

tree::TerminalNode* CypherParser::OC_InQueryCallContext::CALL() {
  return getToken(CypherParser::CALL, 0);
}

std::vector<tree::TerminalNode *> CypherParser::OC_InQueryCallContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_InQueryCallContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}

CypherParser::OC_ExplicitProcedureInvocationContext* CypherParser::OC_InQueryCallContext::oC_ExplicitProcedureInvocation() {
  return getRuleContext<CypherParser::OC_ExplicitProcedureInvocationContext>(0);
}

tree::TerminalNode* CypherParser::OC_InQueryCallContext::YIELD() {
  return getToken(CypherParser::YIELD, 0);
}

CypherParser::OC_YieldItemsContext* CypherParser::OC_InQueryCallContext::oC_YieldItems() {
  return getRuleContext<CypherParser::OC_YieldItemsContext>(0);
}


size_t CypherParser::OC_InQueryCallContext::getRuleIndex() const {
  return CypherParser::RuleOC_InQueryCall;
}

void CypherParser::OC_InQueryCallContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_InQueryCall(this);
}

void CypherParser::OC_InQueryCallContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_InQueryCall(this);
}

CypherParser::OC_InQueryCallContext* CypherParser::oC_InQueryCall() {
  OC_InQueryCallContext *_localctx = _tracker.createInstance<OC_InQueryCallContext>(_ctx, getState());
  enterRule(_localctx, 40, CypherParser::RuleOC_InQueryCall);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(473);
    match(CypherParser::CALL);
    setState(474);
    match(CypherParser::SP);
    setState(475);
    oC_ExplicitProcedureInvocation();
    setState(482);
    _errHandler->sync(this);

    switch (getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 57, _ctx)) {
    case 1: {
      setState(477);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(476);
        match(CypherParser::SP);
      }
      setState(479);
      match(CypherParser::YIELD);
      setState(480);
      match(CypherParser::SP);
      setState(481);
      oC_YieldItems();
      break;
    }

    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_StandaloneCallContext ------------------------------------------------------------------

CypherParser::OC_StandaloneCallContext::OC_StandaloneCallContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

tree::TerminalNode* CypherParser::OC_StandaloneCallContext::CALL() {
  return getToken(CypherParser::CALL, 0);
}

std::vector<tree::TerminalNode *> CypherParser::OC_StandaloneCallContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_StandaloneCallContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}

CypherParser::OC_ExplicitProcedureInvocationContext* CypherParser::OC_StandaloneCallContext::oC_ExplicitProcedureInvocation() {
  return getRuleContext<CypherParser::OC_ExplicitProcedureInvocationContext>(0);
}

CypherParser::OC_ImplicitProcedureInvocationContext* CypherParser::OC_StandaloneCallContext::oC_ImplicitProcedureInvocation() {
  return getRuleContext<CypherParser::OC_ImplicitProcedureInvocationContext>(0);
}

tree::TerminalNode* CypherParser::OC_StandaloneCallContext::YIELD() {
  return getToken(CypherParser::YIELD, 0);
}

CypherParser::OC_YieldItemsContext* CypherParser::OC_StandaloneCallContext::oC_YieldItems() {
  return getRuleContext<CypherParser::OC_YieldItemsContext>(0);
}


size_t CypherParser::OC_StandaloneCallContext::getRuleIndex() const {
  return CypherParser::RuleOC_StandaloneCall;
}

void CypherParser::OC_StandaloneCallContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_StandaloneCall(this);
}

void CypherParser::OC_StandaloneCallContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_StandaloneCall(this);
}

CypherParser::OC_StandaloneCallContext* CypherParser::oC_StandaloneCall() {
  OC_StandaloneCallContext *_localctx = _tracker.createInstance<OC_StandaloneCallContext>(_ctx, getState());
  enterRule(_localctx, 42, CypherParser::RuleOC_StandaloneCall);

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(484);
    match(CypherParser::CALL);
    setState(485);
    match(CypherParser::SP);
    setState(488);
    _errHandler->sync(this);
    switch (getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 58, _ctx)) {
    case 1: {
      setState(486);
      oC_ExplicitProcedureInvocation();
      break;
    }

    case 2: {
      setState(487);
      oC_ImplicitProcedureInvocation();
      break;
    }

    }
    setState(494);
    _errHandler->sync(this);

    switch (getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 59, _ctx)) {
    case 1: {
      setState(490);
      match(CypherParser::SP);
      setState(491);
      match(CypherParser::YIELD);
      setState(492);
      match(CypherParser::SP);
      setState(493);
      oC_YieldItems();
      break;
    }

    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_YieldItemsContext ------------------------------------------------------------------

CypherParser::OC_YieldItemsContext::OC_YieldItemsContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

CypherParser::OC_WhereContext* CypherParser::OC_YieldItemsContext::oC_Where() {
  return getRuleContext<CypherParser::OC_WhereContext>(0);
}

std::vector<CypherParser::OC_YieldItemContext *> CypherParser::OC_YieldItemsContext::oC_YieldItem() {
  return getRuleContexts<CypherParser::OC_YieldItemContext>();
}

CypherParser::OC_YieldItemContext* CypherParser::OC_YieldItemsContext::oC_YieldItem(size_t i) {
  return getRuleContext<CypherParser::OC_YieldItemContext>(i);
}

std::vector<tree::TerminalNode *> CypherParser::OC_YieldItemsContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_YieldItemsContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}


size_t CypherParser::OC_YieldItemsContext::getRuleIndex() const {
  return CypherParser::RuleOC_YieldItems;
}

void CypherParser::OC_YieldItemsContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_YieldItems(this);
}

void CypherParser::OC_YieldItemsContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_YieldItems(this);
}

CypherParser::OC_YieldItemsContext* CypherParser::oC_YieldItems() {
  OC_YieldItemsContext *_localctx = _tracker.createInstance<OC_YieldItemsContext>(_ctx, getState());
  enterRule(_localctx, 44, CypherParser::RuleOC_YieldItems);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    size_t alt;
    enterOuterAlt(_localctx, 1);
    setState(511);
    _errHandler->sync(this);
    switch (_input->LA(1)) {
      case CypherParser::T__4: {
        setState(496);
        match(CypherParser::T__4);
        break;
      }

      case CypherParser::COUNT:
      case CypherParser::ANY:
      case CypherParser::NONE:
      case CypherParser::SINGLE:
      case CypherParser::HexLetter:
      case CypherParser::FILTER:
      case CypherParser::EXTRACT:
      case CypherParser::UnescapedSymbolicName:
      case CypherParser::EscapedSymbolicName: {
        setState(497);
        oC_YieldItem();
        setState(508);
        _errHandler->sync(this);
        alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 62, _ctx);
        while (alt != 2 && alt != atn::ATN::INVALID_ALT_NUMBER) {
          if (alt == 1) {
            setState(499);
            _errHandler->sync(this);

            _la = _input->LA(1);
            if (_la == CypherParser::SP) {
              setState(498);
              match(CypherParser::SP);
            }
            setState(501);
            match(CypherParser::T__1);
            setState(503);
            _errHandler->sync(this);

            _la = _input->LA(1);
            if (_la == CypherParser::SP) {
              setState(502);
              match(CypherParser::SP);
            }
            setState(505);
            oC_YieldItem(); 
          }
          setState(510);
          _errHandler->sync(this);
          alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 62, _ctx);
        }
        break;
      }

    default:
      throw NoViableAltException(this);
    }
    setState(517);
    _errHandler->sync(this);

    switch (getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 65, _ctx)) {
    case 1: {
      setState(514);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(513);
        match(CypherParser::SP);
      }
      setState(516);
      oC_Where();
      break;
    }

    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_YieldItemContext ------------------------------------------------------------------

CypherParser::OC_YieldItemContext::OC_YieldItemContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

CypherParser::OC_VariableContext* CypherParser::OC_YieldItemContext::oC_Variable() {
  return getRuleContext<CypherParser::OC_VariableContext>(0);
}

CypherParser::OC_ProcedureResultFieldContext* CypherParser::OC_YieldItemContext::oC_ProcedureResultField() {
  return getRuleContext<CypherParser::OC_ProcedureResultFieldContext>(0);
}

std::vector<tree::TerminalNode *> CypherParser::OC_YieldItemContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_YieldItemContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}

tree::TerminalNode* CypherParser::OC_YieldItemContext::AS() {
  return getToken(CypherParser::AS, 0);
}


size_t CypherParser::OC_YieldItemContext::getRuleIndex() const {
  return CypherParser::RuleOC_YieldItem;
}

void CypherParser::OC_YieldItemContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_YieldItem(this);
}

void CypherParser::OC_YieldItemContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_YieldItem(this);
}

CypherParser::OC_YieldItemContext* CypherParser::oC_YieldItem() {
  OC_YieldItemContext *_localctx = _tracker.createInstance<OC_YieldItemContext>(_ctx, getState());
  enterRule(_localctx, 46, CypherParser::RuleOC_YieldItem);

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(524);
    _errHandler->sync(this);

    switch (getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 66, _ctx)) {
    case 1: {
      setState(519);
      oC_ProcedureResultField();
      setState(520);
      match(CypherParser::SP);
      setState(521);
      match(CypherParser::AS);
      setState(522);
      match(CypherParser::SP);
      break;
    }

    }
    setState(526);
    oC_Variable();
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_WithContext ------------------------------------------------------------------

CypherParser::OC_WithContext::OC_WithContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

tree::TerminalNode* CypherParser::OC_WithContext::WITH() {
  return getToken(CypherParser::WITH, 0);
}

std::vector<tree::TerminalNode *> CypherParser::OC_WithContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_WithContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}

CypherParser::OC_ReturnBodyContext* CypherParser::OC_WithContext::oC_ReturnBody() {
  return getRuleContext<CypherParser::OC_ReturnBodyContext>(0);
}

tree::TerminalNode* CypherParser::OC_WithContext::DISTINCT() {
  return getToken(CypherParser::DISTINCT, 0);
}

CypherParser::OC_WhereContext* CypherParser::OC_WithContext::oC_Where() {
  return getRuleContext<CypherParser::OC_WhereContext>(0);
}


size_t CypherParser::OC_WithContext::getRuleIndex() const {
  return CypherParser::RuleOC_With;
}

void CypherParser::OC_WithContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_With(this);
}

void CypherParser::OC_WithContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_With(this);
}

CypherParser::OC_WithContext* CypherParser::oC_With() {
  OC_WithContext *_localctx = _tracker.createInstance<OC_WithContext>(_ctx, getState());
  enterRule(_localctx, 48, CypherParser::RuleOC_With);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(528);
    match(CypherParser::WITH);
    setState(533);
    _errHandler->sync(this);

    switch (getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 68, _ctx)) {
    case 1: {
      setState(530);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(529);
        match(CypherParser::SP);
      }
      setState(532);
      match(CypherParser::DISTINCT);
      break;
    }

    }
    setState(535);
    match(CypherParser::SP);
    setState(536);
    oC_ReturnBody();
    setState(541);
    _errHandler->sync(this);

    switch (getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 70, _ctx)) {
    case 1: {
      setState(538);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(537);
        match(CypherParser::SP);
      }
      setState(540);
      oC_Where();
      break;
    }

    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_ReturnContext ------------------------------------------------------------------

CypherParser::OC_ReturnContext::OC_ReturnContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

tree::TerminalNode* CypherParser::OC_ReturnContext::RETURN() {
  return getToken(CypherParser::RETURN, 0);
}

std::vector<tree::TerminalNode *> CypherParser::OC_ReturnContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_ReturnContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}

CypherParser::OC_ReturnBodyContext* CypherParser::OC_ReturnContext::oC_ReturnBody() {
  return getRuleContext<CypherParser::OC_ReturnBodyContext>(0);
}

tree::TerminalNode* CypherParser::OC_ReturnContext::DISTINCT() {
  return getToken(CypherParser::DISTINCT, 0);
}


size_t CypherParser::OC_ReturnContext::getRuleIndex() const {
  return CypherParser::RuleOC_Return;
}

void CypherParser::OC_ReturnContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_Return(this);
}

void CypherParser::OC_ReturnContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_Return(this);
}

CypherParser::OC_ReturnContext* CypherParser::oC_Return() {
  OC_ReturnContext *_localctx = _tracker.createInstance<OC_ReturnContext>(_ctx, getState());
  enterRule(_localctx, 50, CypherParser::RuleOC_Return);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(543);
    match(CypherParser::RETURN);
    setState(548);
    _errHandler->sync(this);

    switch (getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 72, _ctx)) {
    case 1: {
      setState(545);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(544);
        match(CypherParser::SP);
      }
      setState(547);
      match(CypherParser::DISTINCT);
      break;
    }

    }
    setState(550);
    match(CypherParser::SP);
    setState(551);
    oC_ReturnBody();
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_ReturnBodyContext ------------------------------------------------------------------

CypherParser::OC_ReturnBodyContext::OC_ReturnBodyContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

CypherParser::OC_ReturnItemsContext* CypherParser::OC_ReturnBodyContext::oC_ReturnItems() {
  return getRuleContext<CypherParser::OC_ReturnItemsContext>(0);
}

std::vector<tree::TerminalNode *> CypherParser::OC_ReturnBodyContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_ReturnBodyContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}

CypherParser::OC_OrderContext* CypherParser::OC_ReturnBodyContext::oC_Order() {
  return getRuleContext<CypherParser::OC_OrderContext>(0);
}

CypherParser::OC_SkipContext* CypherParser::OC_ReturnBodyContext::oC_Skip() {
  return getRuleContext<CypherParser::OC_SkipContext>(0);
}

CypherParser::OC_LimitContext* CypherParser::OC_ReturnBodyContext::oC_Limit() {
  return getRuleContext<CypherParser::OC_LimitContext>(0);
}


size_t CypherParser::OC_ReturnBodyContext::getRuleIndex() const {
  return CypherParser::RuleOC_ReturnBody;
}

void CypherParser::OC_ReturnBodyContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_ReturnBody(this);
}

void CypherParser::OC_ReturnBodyContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_ReturnBody(this);
}

CypherParser::OC_ReturnBodyContext* CypherParser::oC_ReturnBody() {
  OC_ReturnBodyContext *_localctx = _tracker.createInstance<OC_ReturnBodyContext>(_ctx, getState());
  enterRule(_localctx, 52, CypherParser::RuleOC_ReturnBody);

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(553);
    oC_ReturnItems();
    setState(556);
    _errHandler->sync(this);

    switch (getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 73, _ctx)) {
    case 1: {
      setState(554);
      match(CypherParser::SP);
      setState(555);
      oC_Order();
      break;
    }

    }
    setState(560);
    _errHandler->sync(this);

    switch (getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 74, _ctx)) {
    case 1: {
      setState(558);
      match(CypherParser::SP);
      setState(559);
      oC_Skip();
      break;
    }

    }
    setState(564);
    _errHandler->sync(this);

    switch (getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 75, _ctx)) {
    case 1: {
      setState(562);
      match(CypherParser::SP);
      setState(563);
      oC_Limit();
      break;
    }

    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_ReturnItemsContext ------------------------------------------------------------------

CypherParser::OC_ReturnItemsContext::OC_ReturnItemsContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

std::vector<CypherParser::OC_ReturnItemContext *> CypherParser::OC_ReturnItemsContext::oC_ReturnItem() {
  return getRuleContexts<CypherParser::OC_ReturnItemContext>();
}

CypherParser::OC_ReturnItemContext* CypherParser::OC_ReturnItemsContext::oC_ReturnItem(size_t i) {
  return getRuleContext<CypherParser::OC_ReturnItemContext>(i);
}

std::vector<tree::TerminalNode *> CypherParser::OC_ReturnItemsContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_ReturnItemsContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}


size_t CypherParser::OC_ReturnItemsContext::getRuleIndex() const {
  return CypherParser::RuleOC_ReturnItems;
}

void CypherParser::OC_ReturnItemsContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_ReturnItems(this);
}

void CypherParser::OC_ReturnItemsContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_ReturnItems(this);
}

CypherParser::OC_ReturnItemsContext* CypherParser::oC_ReturnItems() {
  OC_ReturnItemsContext *_localctx = _tracker.createInstance<OC_ReturnItemsContext>(_ctx, getState());
  enterRule(_localctx, 54, CypherParser::RuleOC_ReturnItems);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    size_t alt;
    setState(594);
    _errHandler->sync(this);
    switch (_input->LA(1)) {
      case CypherParser::T__4: {
        enterOuterAlt(_localctx, 1);
        setState(566);
        match(CypherParser::T__4);
        setState(577);
        _errHandler->sync(this);
        alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 78, _ctx);
        while (alt != 2 && alt != atn::ATN::INVALID_ALT_NUMBER) {
          if (alt == 1) {
            setState(568);
            _errHandler->sync(this);

            _la = _input->LA(1);
            if (_la == CypherParser::SP) {
              setState(567);
              match(CypherParser::SP);
            }
            setState(570);
            match(CypherParser::T__1);
            setState(572);
            _errHandler->sync(this);

            _la = _input->LA(1);
            if (_la == CypherParser::SP) {
              setState(571);
              match(CypherParser::SP);
            }
            setState(574);
            oC_ReturnItem(); 
          }
          setState(579);
          _errHandler->sync(this);
          alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 78, _ctx);
        }
        break;
      }

      case CypherParser::T__5:
      case CypherParser::T__7:
      case CypherParser::T__12:
      case CypherParser::T__13:
      case CypherParser::T__23:
      case CypherParser::T__25:
      case CypherParser::ALL:
      case CypherParser::NOT:
      case CypherParser::NULLOP:
      case CypherParser::COUNT:
      case CypherParser::ANY:
      case CypherParser::NONE:
      case CypherParser::SINGLE:
      case CypherParser::TRUE:
      case CypherParser::FALSE:
      case CypherParser::EXISTS:
      case CypherParser::CASE:
      case CypherParser::StringLiteral:
      case CypherParser::HexInteger:
      case CypherParser::DecimalInteger:
      case CypherParser::OctalInteger:
      case CypherParser::HexLetter:
      case CypherParser::ExponentDecimalReal:
      case CypherParser::RegularDecimalReal:
      case CypherParser::FILTER:
      case CypherParser::EXTRACT:
      case CypherParser::UnescapedSymbolicName:
      case CypherParser::EscapedSymbolicName: {
        enterOuterAlt(_localctx, 2);
        setState(580);
        oC_ReturnItem();
        setState(591);
        _errHandler->sync(this);
        alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 81, _ctx);
        while (alt != 2 && alt != atn::ATN::INVALID_ALT_NUMBER) {
          if (alt == 1) {
            setState(582);
            _errHandler->sync(this);

            _la = _input->LA(1);
            if (_la == CypherParser::SP) {
              setState(581);
              match(CypherParser::SP);
            }
            setState(584);
            match(CypherParser::T__1);
            setState(586);
            _errHandler->sync(this);

            _la = _input->LA(1);
            if (_la == CypherParser::SP) {
              setState(585);
              match(CypherParser::SP);
            }
            setState(588);
            oC_ReturnItem(); 
          }
          setState(593);
          _errHandler->sync(this);
          alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 81, _ctx);
        }
        break;
      }

    default:
      throw NoViableAltException(this);
    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_ReturnItemContext ------------------------------------------------------------------

CypherParser::OC_ReturnItemContext::OC_ReturnItemContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

CypherParser::OC_ExpressionContext* CypherParser::OC_ReturnItemContext::oC_Expression() {
  return getRuleContext<CypherParser::OC_ExpressionContext>(0);
}

std::vector<tree::TerminalNode *> CypherParser::OC_ReturnItemContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_ReturnItemContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}

tree::TerminalNode* CypherParser::OC_ReturnItemContext::AS() {
  return getToken(CypherParser::AS, 0);
}

CypherParser::OC_VariableContext* CypherParser::OC_ReturnItemContext::oC_Variable() {
  return getRuleContext<CypherParser::OC_VariableContext>(0);
}


size_t CypherParser::OC_ReturnItemContext::getRuleIndex() const {
  return CypherParser::RuleOC_ReturnItem;
}

void CypherParser::OC_ReturnItemContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_ReturnItem(this);
}

void CypherParser::OC_ReturnItemContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_ReturnItem(this);
}

CypherParser::OC_ReturnItemContext* CypherParser::oC_ReturnItem() {
  OC_ReturnItemContext *_localctx = _tracker.createInstance<OC_ReturnItemContext>(_ctx, getState());
  enterRule(_localctx, 56, CypherParser::RuleOC_ReturnItem);

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    setState(603);
    _errHandler->sync(this);
    switch (getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 83, _ctx)) {
    case 1: {
      enterOuterAlt(_localctx, 1);
      setState(596);
      oC_Expression();
      setState(597);
      match(CypherParser::SP);
      setState(598);
      match(CypherParser::AS);
      setState(599);
      match(CypherParser::SP);
      setState(600);
      oC_Variable();
      break;
    }

    case 2: {
      enterOuterAlt(_localctx, 2);
      setState(602);
      oC_Expression();
      break;
    }

    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_OrderContext ------------------------------------------------------------------

CypherParser::OC_OrderContext::OC_OrderContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

tree::TerminalNode* CypherParser::OC_OrderContext::ORDER() {
  return getToken(CypherParser::ORDER, 0);
}

std::vector<tree::TerminalNode *> CypherParser::OC_OrderContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_OrderContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}

tree::TerminalNode* CypherParser::OC_OrderContext::BY() {
  return getToken(CypherParser::BY, 0);
}

std::vector<CypherParser::OC_SortItemContext *> CypherParser::OC_OrderContext::oC_SortItem() {
  return getRuleContexts<CypherParser::OC_SortItemContext>();
}

CypherParser::OC_SortItemContext* CypherParser::OC_OrderContext::oC_SortItem(size_t i) {
  return getRuleContext<CypherParser::OC_SortItemContext>(i);
}


size_t CypherParser::OC_OrderContext::getRuleIndex() const {
  return CypherParser::RuleOC_Order;
}

void CypherParser::OC_OrderContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_Order(this);
}

void CypherParser::OC_OrderContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_Order(this);
}

CypherParser::OC_OrderContext* CypherParser::oC_Order() {
  OC_OrderContext *_localctx = _tracker.createInstance<OC_OrderContext>(_ctx, getState());
  enterRule(_localctx, 58, CypherParser::RuleOC_Order);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(605);
    match(CypherParser::ORDER);
    setState(606);
    match(CypherParser::SP);
    setState(607);
    match(CypherParser::BY);
    setState(608);
    match(CypherParser::SP);
    setState(609);
    oC_SortItem();
    setState(617);
    _errHandler->sync(this);
    _la = _input->LA(1);
    while (_la == CypherParser::T__1) {
      setState(610);
      match(CypherParser::T__1);
      setState(612);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(611);
        match(CypherParser::SP);
      }
      setState(614);
      oC_SortItem();
      setState(619);
      _errHandler->sync(this);
      _la = _input->LA(1);
    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_SkipContext ------------------------------------------------------------------

CypherParser::OC_SkipContext::OC_SkipContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

tree::TerminalNode* CypherParser::OC_SkipContext::L_SKIP() {
  return getToken(CypherParser::L_SKIP, 0);
}

tree::TerminalNode* CypherParser::OC_SkipContext::SP() {
  return getToken(CypherParser::SP, 0);
}

CypherParser::OC_ExpressionContext* CypherParser::OC_SkipContext::oC_Expression() {
  return getRuleContext<CypherParser::OC_ExpressionContext>(0);
}


size_t CypherParser::OC_SkipContext::getRuleIndex() const {
  return CypherParser::RuleOC_Skip;
}

void CypherParser::OC_SkipContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_Skip(this);
}

void CypherParser::OC_SkipContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_Skip(this);
}

CypherParser::OC_SkipContext* CypherParser::oC_Skip() {
  OC_SkipContext *_localctx = _tracker.createInstance<OC_SkipContext>(_ctx, getState());
  enterRule(_localctx, 60, CypherParser::RuleOC_Skip);

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(620);
    match(CypherParser::L_SKIP);
    setState(621);
    match(CypherParser::SP);
    setState(622);
    oC_Expression();
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_LimitContext ------------------------------------------------------------------

CypherParser::OC_LimitContext::OC_LimitContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

tree::TerminalNode* CypherParser::OC_LimitContext::LIMIT() {
  return getToken(CypherParser::LIMIT, 0);
}

tree::TerminalNode* CypherParser::OC_LimitContext::SP() {
  return getToken(CypherParser::SP, 0);
}

CypherParser::OC_ExpressionContext* CypherParser::OC_LimitContext::oC_Expression() {
  return getRuleContext<CypherParser::OC_ExpressionContext>(0);
}


size_t CypherParser::OC_LimitContext::getRuleIndex() const {
  return CypherParser::RuleOC_Limit;
}

void CypherParser::OC_LimitContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_Limit(this);
}

void CypherParser::OC_LimitContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_Limit(this);
}

CypherParser::OC_LimitContext* CypherParser::oC_Limit() {
  OC_LimitContext *_localctx = _tracker.createInstance<OC_LimitContext>(_ctx, getState());
  enterRule(_localctx, 62, CypherParser::RuleOC_Limit);

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(624);
    match(CypherParser::LIMIT);
    setState(625);
    match(CypherParser::SP);
    setState(626);
    oC_Expression();
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_SortItemContext ------------------------------------------------------------------

CypherParser::OC_SortItemContext::OC_SortItemContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

CypherParser::OC_ExpressionContext* CypherParser::OC_SortItemContext::oC_Expression() {
  return getRuleContext<CypherParser::OC_ExpressionContext>(0);
}

tree::TerminalNode* CypherParser::OC_SortItemContext::ASCENDING() {
  return getToken(CypherParser::ASCENDING, 0);
}

tree::TerminalNode* CypherParser::OC_SortItemContext::ASC() {
  return getToken(CypherParser::ASC, 0);
}

tree::TerminalNode* CypherParser::OC_SortItemContext::DESCENDING() {
  return getToken(CypherParser::DESCENDING, 0);
}

tree::TerminalNode* CypherParser::OC_SortItemContext::DESC() {
  return getToken(CypherParser::DESC, 0);
}

tree::TerminalNode* CypherParser::OC_SortItemContext::SP() {
  return getToken(CypherParser::SP, 0);
}


size_t CypherParser::OC_SortItemContext::getRuleIndex() const {
  return CypherParser::RuleOC_SortItem;
}

void CypherParser::OC_SortItemContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_SortItem(this);
}

void CypherParser::OC_SortItemContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_SortItem(this);
}

CypherParser::OC_SortItemContext* CypherParser::oC_SortItem() {
  OC_SortItemContext *_localctx = _tracker.createInstance<OC_SortItemContext>(_ctx, getState());
  enterRule(_localctx, 64, CypherParser::RuleOC_SortItem);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(628);
    oC_Expression();
    setState(633);
    _errHandler->sync(this);

    switch (getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 87, _ctx)) {
    case 1: {
      setState(630);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(629);
        match(CypherParser::SP);
      }
      setState(632);
      _la = _input->LA(1);
      if (!(((((_la - 68) & ~ 0x3fULL) == 0) &&
        ((1ULL << (_la - 68)) & ((1ULL << (CypherParser::ASCENDING - 68))
        | (1ULL << (CypherParser::ASC - 68))
        | (1ULL << (CypherParser::DESCENDING - 68))
        | (1ULL << (CypherParser::DESC - 68)))) != 0))) {
      _errHandler->recoverInline(this);
      }
      else {
        _errHandler->reportMatch(this);
        consume();
      }
      break;
    }

    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_WhereContext ------------------------------------------------------------------

CypherParser::OC_WhereContext::OC_WhereContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

tree::TerminalNode* CypherParser::OC_WhereContext::WHERE() {
  return getToken(CypherParser::WHERE, 0);
}

tree::TerminalNode* CypherParser::OC_WhereContext::SP() {
  return getToken(CypherParser::SP, 0);
}

CypherParser::OC_ExpressionContext* CypherParser::OC_WhereContext::oC_Expression() {
  return getRuleContext<CypherParser::OC_ExpressionContext>(0);
}


size_t CypherParser::OC_WhereContext::getRuleIndex() const {
  return CypherParser::RuleOC_Where;
}

void CypherParser::OC_WhereContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_Where(this);
}

void CypherParser::OC_WhereContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_Where(this);
}

CypherParser::OC_WhereContext* CypherParser::oC_Where() {
  OC_WhereContext *_localctx = _tracker.createInstance<OC_WhereContext>(_ctx, getState());
  enterRule(_localctx, 66, CypherParser::RuleOC_Where);

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(635);
    match(CypherParser::WHERE);
    setState(636);
    match(CypherParser::SP);
    setState(637);
    oC_Expression();
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_PatternContext ------------------------------------------------------------------

CypherParser::OC_PatternContext::OC_PatternContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

std::vector<CypherParser::OC_PatternPartContext *> CypherParser::OC_PatternContext::oC_PatternPart() {
  return getRuleContexts<CypherParser::OC_PatternPartContext>();
}

CypherParser::OC_PatternPartContext* CypherParser::OC_PatternContext::oC_PatternPart(size_t i) {
  return getRuleContext<CypherParser::OC_PatternPartContext>(i);
}

std::vector<tree::TerminalNode *> CypherParser::OC_PatternContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_PatternContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}


size_t CypherParser::OC_PatternContext::getRuleIndex() const {
  return CypherParser::RuleOC_Pattern;
}

void CypherParser::OC_PatternContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_Pattern(this);
}

void CypherParser::OC_PatternContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_Pattern(this);
}

CypherParser::OC_PatternContext* CypherParser::oC_Pattern() {
  OC_PatternContext *_localctx = _tracker.createInstance<OC_PatternContext>(_ctx, getState());
  enterRule(_localctx, 68, CypherParser::RuleOC_Pattern);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    size_t alt;
    enterOuterAlt(_localctx, 1);
    setState(639);
    oC_PatternPart();
    setState(650);
    _errHandler->sync(this);
    alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 90, _ctx);
    while (alt != 2 && alt != atn::ATN::INVALID_ALT_NUMBER) {
      if (alt == 1) {
        setState(641);
        _errHandler->sync(this);

        _la = _input->LA(1);
        if (_la == CypherParser::SP) {
          setState(640);
          match(CypherParser::SP);
        }
        setState(643);
        match(CypherParser::T__1);
        setState(645);
        _errHandler->sync(this);

        _la = _input->LA(1);
        if (_la == CypherParser::SP) {
          setState(644);
          match(CypherParser::SP);
        }
        setState(647);
        oC_PatternPart(); 
      }
      setState(652);
      _errHandler->sync(this);
      alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 90, _ctx);
    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_PatternPartContext ------------------------------------------------------------------

CypherParser::OC_PatternPartContext::OC_PatternPartContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

CypherParser::OC_VariableContext* CypherParser::OC_PatternPartContext::oC_Variable() {
  return getRuleContext<CypherParser::OC_VariableContext>(0);
}

CypherParser::OC_AnonymousPatternPartContext* CypherParser::OC_PatternPartContext::oC_AnonymousPatternPart() {
  return getRuleContext<CypherParser::OC_AnonymousPatternPartContext>(0);
}

std::vector<tree::TerminalNode *> CypherParser::OC_PatternPartContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_PatternPartContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}


size_t CypherParser::OC_PatternPartContext::getRuleIndex() const {
  return CypherParser::RuleOC_PatternPart;
}

void CypherParser::OC_PatternPartContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_PatternPart(this);
}

void CypherParser::OC_PatternPartContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_PatternPart(this);
}

CypherParser::OC_PatternPartContext* CypherParser::oC_PatternPart() {
  OC_PatternPartContext *_localctx = _tracker.createInstance<OC_PatternPartContext>(_ctx, getState());
  enterRule(_localctx, 70, CypherParser::RuleOC_PatternPart);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    setState(664);
    _errHandler->sync(this);
    switch (_input->LA(1)) {
      case CypherParser::COUNT:
      case CypherParser::ANY:
      case CypherParser::NONE:
      case CypherParser::SINGLE:
      case CypherParser::HexLetter:
      case CypherParser::FILTER:
      case CypherParser::EXTRACT:
      case CypherParser::UnescapedSymbolicName:
      case CypherParser::EscapedSymbolicName: {
        enterOuterAlt(_localctx, 1);
        setState(653);
        oC_Variable();
        setState(655);
        _errHandler->sync(this);

        _la = _input->LA(1);
        if (_la == CypherParser::SP) {
          setState(654);
          match(CypherParser::SP);
        }
        setState(657);
        match(CypherParser::T__2);
        setState(659);
        _errHandler->sync(this);

        _la = _input->LA(1);
        if (_la == CypherParser::SP) {
          setState(658);
          match(CypherParser::SP);
        }
        setState(661);
        oC_AnonymousPatternPart();
        break;
      }

      case CypherParser::T__5: {
        enterOuterAlt(_localctx, 2);
        setState(663);
        oC_AnonymousPatternPart();
        break;
      }

    default:
      throw NoViableAltException(this);
    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_AnonymousPatternPartContext ------------------------------------------------------------------

CypherParser::OC_AnonymousPatternPartContext::OC_AnonymousPatternPartContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

CypherParser::OC_PatternElementContext* CypherParser::OC_AnonymousPatternPartContext::oC_PatternElement() {
  return getRuleContext<CypherParser::OC_PatternElementContext>(0);
}


size_t CypherParser::OC_AnonymousPatternPartContext::getRuleIndex() const {
  return CypherParser::RuleOC_AnonymousPatternPart;
}

void CypherParser::OC_AnonymousPatternPartContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_AnonymousPatternPart(this);
}

void CypherParser::OC_AnonymousPatternPartContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_AnonymousPatternPart(this);
}

CypherParser::OC_AnonymousPatternPartContext* CypherParser::oC_AnonymousPatternPart() {
  OC_AnonymousPatternPartContext *_localctx = _tracker.createInstance<OC_AnonymousPatternPartContext>(_ctx, getState());
  enterRule(_localctx, 72, CypherParser::RuleOC_AnonymousPatternPart);

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(666);
    oC_PatternElement();
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_PatternElementContext ------------------------------------------------------------------

CypherParser::OC_PatternElementContext::OC_PatternElementContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

CypherParser::OC_NodePatternContext* CypherParser::OC_PatternElementContext::oC_NodePattern() {
  return getRuleContext<CypherParser::OC_NodePatternContext>(0);
}

std::vector<CypherParser::OC_PatternElementChainContext *> CypherParser::OC_PatternElementContext::oC_PatternElementChain() {
  return getRuleContexts<CypherParser::OC_PatternElementChainContext>();
}

CypherParser::OC_PatternElementChainContext* CypherParser::OC_PatternElementContext::oC_PatternElementChain(size_t i) {
  return getRuleContext<CypherParser::OC_PatternElementChainContext>(i);
}

std::vector<tree::TerminalNode *> CypherParser::OC_PatternElementContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_PatternElementContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}

CypherParser::OC_PatternElementContext* CypherParser::OC_PatternElementContext::oC_PatternElement() {
  return getRuleContext<CypherParser::OC_PatternElementContext>(0);
}


size_t CypherParser::OC_PatternElementContext::getRuleIndex() const {
  return CypherParser::RuleOC_PatternElement;
}

void CypherParser::OC_PatternElementContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_PatternElement(this);
}

void CypherParser::OC_PatternElementContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_PatternElement(this);
}

CypherParser::OC_PatternElementContext* CypherParser::oC_PatternElement() {
  OC_PatternElementContext *_localctx = _tracker.createInstance<OC_PatternElementContext>(_ctx, getState());
  enterRule(_localctx, 74, CypherParser::RuleOC_PatternElement);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    size_t alt;
    setState(682);
    _errHandler->sync(this);
    switch (getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 96, _ctx)) {
    case 1: {
      enterOuterAlt(_localctx, 1);
      setState(668);
      oC_NodePattern();
      setState(675);
      _errHandler->sync(this);
      alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 95, _ctx);
      while (alt != 2 && alt != atn::ATN::INVALID_ALT_NUMBER) {
        if (alt == 1) {
          setState(670);
          _errHandler->sync(this);

          _la = _input->LA(1);
          if (_la == CypherParser::SP) {
            setState(669);
            match(CypherParser::SP);
          }
          setState(672);
          oC_PatternElementChain(); 
        }
        setState(677);
        _errHandler->sync(this);
        alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 95, _ctx);
      }
      break;
    }

    case 2: {
      enterOuterAlt(_localctx, 2);
      setState(678);
      match(CypherParser::T__5);
      setState(679);
      oC_PatternElement();
      setState(680);
      match(CypherParser::T__6);
      break;
    }

    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_NodePatternContext ------------------------------------------------------------------

CypherParser::OC_NodePatternContext::OC_NodePatternContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

std::vector<tree::TerminalNode *> CypherParser::OC_NodePatternContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_NodePatternContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}

CypherParser::OC_VariableContext* CypherParser::OC_NodePatternContext::oC_Variable() {
  return getRuleContext<CypherParser::OC_VariableContext>(0);
}

CypherParser::OC_NodeLabelsContext* CypherParser::OC_NodePatternContext::oC_NodeLabels() {
  return getRuleContext<CypherParser::OC_NodeLabelsContext>(0);
}

CypherParser::OC_PropertiesContext* CypherParser::OC_NodePatternContext::oC_Properties() {
  return getRuleContext<CypherParser::OC_PropertiesContext>(0);
}


size_t CypherParser::OC_NodePatternContext::getRuleIndex() const {
  return CypherParser::RuleOC_NodePattern;
}

void CypherParser::OC_NodePatternContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_NodePattern(this);
}

void CypherParser::OC_NodePatternContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_NodePattern(this);
}

CypherParser::OC_NodePatternContext* CypherParser::oC_NodePattern() {
  OC_NodePatternContext *_localctx = _tracker.createInstance<OC_NodePatternContext>(_ctx, getState());
  enterRule(_localctx, 76, CypherParser::RuleOC_NodePattern);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(684);
    match(CypherParser::T__5);
    setState(686);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (_la == CypherParser::SP) {
      setState(685);
      match(CypherParser::SP);
    }
    setState(692);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (((((_la - 83) & ~ 0x3fULL) == 0) &&
      ((1ULL << (_la - 83)) & ((1ULL << (CypherParser::COUNT - 83))
      | (1ULL << (CypherParser::ANY - 83))
      | (1ULL << (CypherParser::NONE - 83))
      | (1ULL << (CypherParser::SINGLE - 83))
      | (1ULL << (CypherParser::HexLetter - 83))
      | (1ULL << (CypherParser::FILTER - 83))
      | (1ULL << (CypherParser::EXTRACT - 83))
      | (1ULL << (CypherParser::UnescapedSymbolicName - 83))
      | (1ULL << (CypherParser::EscapedSymbolicName - 83)))) != 0)) {
      setState(688);
      oC_Variable();
      setState(690);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(689);
        match(CypherParser::SP);
      }
    }
    setState(698);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (_la == CypherParser::T__9) {
      setState(694);
      oC_NodeLabels();
      setState(696);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(695);
        match(CypherParser::SP);
      }
    }
    setState(704);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (_la == CypherParser::T__23

    || _la == CypherParser::T__25) {
      setState(700);
      oC_Properties();
      setState(702);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(701);
        match(CypherParser::SP);
      }
    }
    setState(706);
    match(CypherParser::T__6);
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_PatternElementChainContext ------------------------------------------------------------------

CypherParser::OC_PatternElementChainContext::OC_PatternElementChainContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

CypherParser::OC_RelationshipPatternContext* CypherParser::OC_PatternElementChainContext::oC_RelationshipPattern() {
  return getRuleContext<CypherParser::OC_RelationshipPatternContext>(0);
}

CypherParser::OC_NodePatternContext* CypherParser::OC_PatternElementChainContext::oC_NodePattern() {
  return getRuleContext<CypherParser::OC_NodePatternContext>(0);
}

tree::TerminalNode* CypherParser::OC_PatternElementChainContext::SP() {
  return getToken(CypherParser::SP, 0);
}


size_t CypherParser::OC_PatternElementChainContext::getRuleIndex() const {
  return CypherParser::RuleOC_PatternElementChain;
}

void CypherParser::OC_PatternElementChainContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_PatternElementChain(this);
}

void CypherParser::OC_PatternElementChainContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_PatternElementChain(this);
}

CypherParser::OC_PatternElementChainContext* CypherParser::oC_PatternElementChain() {
  OC_PatternElementChainContext *_localctx = _tracker.createInstance<OC_PatternElementChainContext>(_ctx, getState());
  enterRule(_localctx, 78, CypherParser::RuleOC_PatternElementChain);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(708);
    oC_RelationshipPattern();
    setState(710);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (_la == CypherParser::SP) {
      setState(709);
      match(CypherParser::SP);
    }
    setState(712);
    oC_NodePattern();
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_RelationshipPatternContext ------------------------------------------------------------------

CypherParser::OC_RelationshipPatternContext::OC_RelationshipPatternContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

CypherParser::OC_LeftArrowHeadContext* CypherParser::OC_RelationshipPatternContext::oC_LeftArrowHead() {
  return getRuleContext<CypherParser::OC_LeftArrowHeadContext>(0);
}

std::vector<CypherParser::OC_DashContext *> CypherParser::OC_RelationshipPatternContext::oC_Dash() {
  return getRuleContexts<CypherParser::OC_DashContext>();
}

CypherParser::OC_DashContext* CypherParser::OC_RelationshipPatternContext::oC_Dash(size_t i) {
  return getRuleContext<CypherParser::OC_DashContext>(i);
}

CypherParser::OC_RightArrowHeadContext* CypherParser::OC_RelationshipPatternContext::oC_RightArrowHead() {
  return getRuleContext<CypherParser::OC_RightArrowHeadContext>(0);
}

std::vector<tree::TerminalNode *> CypherParser::OC_RelationshipPatternContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_RelationshipPatternContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}

CypherParser::OC_RelationshipDetailContext* CypherParser::OC_RelationshipPatternContext::oC_RelationshipDetail() {
  return getRuleContext<CypherParser::OC_RelationshipDetailContext>(0);
}


size_t CypherParser::OC_RelationshipPatternContext::getRuleIndex() const {
  return CypherParser::RuleOC_RelationshipPattern;
}

void CypherParser::OC_RelationshipPatternContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_RelationshipPattern(this);
}

void CypherParser::OC_RelationshipPatternContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_RelationshipPattern(this);
}

CypherParser::OC_RelationshipPatternContext* CypherParser::oC_RelationshipPattern() {
  OC_RelationshipPatternContext *_localctx = _tracker.createInstance<OC_RelationshipPatternContext>(_ctx, getState());
  enterRule(_localctx, 80, CypherParser::RuleOC_RelationshipPattern);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    setState(778);
    _errHandler->sync(this);
    switch (getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 121, _ctx)) {
    case 1: {
      enterOuterAlt(_localctx, 1);
      setState(714);
      oC_LeftArrowHead();
      setState(716);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(715);
        match(CypherParser::SP);
      }
      setState(718);
      oC_Dash();
      setState(720);
      _errHandler->sync(this);

      switch (getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 106, _ctx)) {
      case 1: {
        setState(719);
        match(CypherParser::SP);
        break;
      }

      }
      setState(723);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::T__7) {
        setState(722);
        oC_RelationshipDetail();
      }
      setState(726);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(725);
        match(CypherParser::SP);
      }
      setState(728);
      oC_Dash();
      setState(730);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(729);
        match(CypherParser::SP);
      }
      setState(732);
      oC_RightArrowHead();
      break;
    }

    case 2: {
      enterOuterAlt(_localctx, 2);
      setState(734);
      oC_LeftArrowHead();
      setState(736);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(735);
        match(CypherParser::SP);
      }
      setState(738);
      oC_Dash();
      setState(740);
      _errHandler->sync(this);

      switch (getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 111, _ctx)) {
      case 1: {
        setState(739);
        match(CypherParser::SP);
        break;
      }

      }
      setState(743);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::T__7) {
        setState(742);
        oC_RelationshipDetail();
      }
      setState(746);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(745);
        match(CypherParser::SP);
      }
      setState(748);
      oC_Dash();
      break;
    }

    case 3: {
      enterOuterAlt(_localctx, 3);
      setState(750);
      oC_Dash();
      setState(752);
      _errHandler->sync(this);

      switch (getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 114, _ctx)) {
      case 1: {
        setState(751);
        match(CypherParser::SP);
        break;
      }

      }
      setState(755);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::T__7) {
        setState(754);
        oC_RelationshipDetail();
      }
      setState(758);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(757);
        match(CypherParser::SP);
      }
      setState(760);
      oC_Dash();
      setState(762);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(761);
        match(CypherParser::SP);
      }
      setState(764);
      oC_RightArrowHead();
      break;
    }

    case 4: {
      enterOuterAlt(_localctx, 4);
      setState(766);
      oC_Dash();
      setState(768);
      _errHandler->sync(this);

      switch (getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 118, _ctx)) {
      case 1: {
        setState(767);
        match(CypherParser::SP);
        break;
      }

      }
      setState(771);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::T__7) {
        setState(770);
        oC_RelationshipDetail();
      }
      setState(774);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(773);
        match(CypherParser::SP);
      }
      setState(776);
      oC_Dash();
      break;
    }

    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_RelationshipDetailContext ------------------------------------------------------------------

CypherParser::OC_RelationshipDetailContext::OC_RelationshipDetailContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

std::vector<tree::TerminalNode *> CypherParser::OC_RelationshipDetailContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_RelationshipDetailContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}

CypherParser::OC_VariableContext* CypherParser::OC_RelationshipDetailContext::oC_Variable() {
  return getRuleContext<CypherParser::OC_VariableContext>(0);
}

CypherParser::OC_RelationshipTypesContext* CypherParser::OC_RelationshipDetailContext::oC_RelationshipTypes() {
  return getRuleContext<CypherParser::OC_RelationshipTypesContext>(0);
}

CypherParser::OC_RangeLiteralContext* CypherParser::OC_RelationshipDetailContext::oC_RangeLiteral() {
  return getRuleContext<CypherParser::OC_RangeLiteralContext>(0);
}

CypherParser::OC_PropertiesContext* CypherParser::OC_RelationshipDetailContext::oC_Properties() {
  return getRuleContext<CypherParser::OC_PropertiesContext>(0);
}


size_t CypherParser::OC_RelationshipDetailContext::getRuleIndex() const {
  return CypherParser::RuleOC_RelationshipDetail;
}

void CypherParser::OC_RelationshipDetailContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_RelationshipDetail(this);
}

void CypherParser::OC_RelationshipDetailContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_RelationshipDetail(this);
}

CypherParser::OC_RelationshipDetailContext* CypherParser::oC_RelationshipDetail() {
  OC_RelationshipDetailContext *_localctx = _tracker.createInstance<OC_RelationshipDetailContext>(_ctx, getState());
  enterRule(_localctx, 82, CypherParser::RuleOC_RelationshipDetail);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(780);
    match(CypherParser::T__7);
    setState(782);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (_la == CypherParser::SP) {
      setState(781);
      match(CypherParser::SP);
    }
    setState(788);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (((((_la - 83) & ~ 0x3fULL) == 0) &&
      ((1ULL << (_la - 83)) & ((1ULL << (CypherParser::COUNT - 83))
      | (1ULL << (CypherParser::ANY - 83))
      | (1ULL << (CypherParser::NONE - 83))
      | (1ULL << (CypherParser::SINGLE - 83))
      | (1ULL << (CypherParser::HexLetter - 83))
      | (1ULL << (CypherParser::FILTER - 83))
      | (1ULL << (CypherParser::EXTRACT - 83))
      | (1ULL << (CypherParser::UnescapedSymbolicName - 83))
      | (1ULL << (CypherParser::EscapedSymbolicName - 83)))) != 0)) {
      setState(784);
      oC_Variable();
      setState(786);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(785);
        match(CypherParser::SP);
      }
    }
    setState(794);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (_la == CypherParser::T__9) {
      setState(790);
      oC_RelationshipTypes();
      setState(792);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(791);
        match(CypherParser::SP);
      }
    }
    setState(797);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (_la == CypherParser::T__4) {
      setState(796);
      oC_RangeLiteral();
    }
    setState(803);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (_la == CypherParser::T__23

    || _la == CypherParser::T__25) {
      setState(799);
      oC_Properties();
      setState(801);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(800);
        match(CypherParser::SP);
      }
    }
    setState(805);
    match(CypherParser::T__8);
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_PropertiesContext ------------------------------------------------------------------

CypherParser::OC_PropertiesContext::OC_PropertiesContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

CypherParser::OC_MapLiteralContext* CypherParser::OC_PropertiesContext::oC_MapLiteral() {
  return getRuleContext<CypherParser::OC_MapLiteralContext>(0);
}

CypherParser::OC_ParameterContext* CypherParser::OC_PropertiesContext::oC_Parameter() {
  return getRuleContext<CypherParser::OC_ParameterContext>(0);
}


size_t CypherParser::OC_PropertiesContext::getRuleIndex() const {
  return CypherParser::RuleOC_Properties;
}

void CypherParser::OC_PropertiesContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_Properties(this);
}

void CypherParser::OC_PropertiesContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_Properties(this);
}

CypherParser::OC_PropertiesContext* CypherParser::oC_Properties() {
  OC_PropertiesContext *_localctx = _tracker.createInstance<OC_PropertiesContext>(_ctx, getState());
  enterRule(_localctx, 84, CypherParser::RuleOC_Properties);

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    setState(809);
    _errHandler->sync(this);
    switch (_input->LA(1)) {
      case CypherParser::T__23: {
        enterOuterAlt(_localctx, 1);
        setState(807);
        oC_MapLiteral();
        break;
      }

      case CypherParser::T__25: {
        enterOuterAlt(_localctx, 2);
        setState(808);
        oC_Parameter();
        break;
      }

    default:
      throw NoViableAltException(this);
    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_RelationshipTypesContext ------------------------------------------------------------------

CypherParser::OC_RelationshipTypesContext::OC_RelationshipTypesContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

std::vector<CypherParser::OC_RelTypeNameContext *> CypherParser::OC_RelationshipTypesContext::oC_RelTypeName() {
  return getRuleContexts<CypherParser::OC_RelTypeNameContext>();
}

CypherParser::OC_RelTypeNameContext* CypherParser::OC_RelationshipTypesContext::oC_RelTypeName(size_t i) {
  return getRuleContext<CypherParser::OC_RelTypeNameContext>(i);
}

std::vector<tree::TerminalNode *> CypherParser::OC_RelationshipTypesContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_RelationshipTypesContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}


size_t CypherParser::OC_RelationshipTypesContext::getRuleIndex() const {
  return CypherParser::RuleOC_RelationshipTypes;
}

void CypherParser::OC_RelationshipTypesContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_RelationshipTypes(this);
}

void CypherParser::OC_RelationshipTypesContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_RelationshipTypes(this);
}

CypherParser::OC_RelationshipTypesContext* CypherParser::oC_RelationshipTypes() {
  OC_RelationshipTypesContext *_localctx = _tracker.createInstance<OC_RelationshipTypesContext>(_ctx, getState());
  enterRule(_localctx, 86, CypherParser::RuleOC_RelationshipTypes);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    size_t alt;
    enterOuterAlt(_localctx, 1);
    setState(811);
    match(CypherParser::T__9);
    setState(813);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (_la == CypherParser::SP) {
      setState(812);
      match(CypherParser::SP);
    }
    setState(815);
    oC_RelTypeName();
    setState(829);
    _errHandler->sync(this);
    alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 135, _ctx);
    while (alt != 2 && alt != atn::ATN::INVALID_ALT_NUMBER) {
      if (alt == 1) {
        setState(817);
        _errHandler->sync(this);

        _la = _input->LA(1);
        if (_la == CypherParser::SP) {
          setState(816);
          match(CypherParser::SP);
        }
        setState(819);
        match(CypherParser::T__10);
        setState(821);
        _errHandler->sync(this);

        _la = _input->LA(1);
        if (_la == CypherParser::T__9) {
          setState(820);
          match(CypherParser::T__9);
        }
        setState(824);
        _errHandler->sync(this);

        _la = _input->LA(1);
        if (_la == CypherParser::SP) {
          setState(823);
          match(CypherParser::SP);
        }
        setState(826);
        oC_RelTypeName(); 
      }
      setState(831);
      _errHandler->sync(this);
      alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 135, _ctx);
    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_NodeLabelsContext ------------------------------------------------------------------

CypherParser::OC_NodeLabelsContext::OC_NodeLabelsContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

std::vector<CypherParser::OC_NodeLabelContext *> CypherParser::OC_NodeLabelsContext::oC_NodeLabel() {
  return getRuleContexts<CypherParser::OC_NodeLabelContext>();
}

CypherParser::OC_NodeLabelContext* CypherParser::OC_NodeLabelsContext::oC_NodeLabel(size_t i) {
  return getRuleContext<CypherParser::OC_NodeLabelContext>(i);
}

std::vector<tree::TerminalNode *> CypherParser::OC_NodeLabelsContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_NodeLabelsContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}


size_t CypherParser::OC_NodeLabelsContext::getRuleIndex() const {
  return CypherParser::RuleOC_NodeLabels;
}

void CypherParser::OC_NodeLabelsContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_NodeLabels(this);
}

void CypherParser::OC_NodeLabelsContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_NodeLabels(this);
}

CypherParser::OC_NodeLabelsContext* CypherParser::oC_NodeLabels() {
  OC_NodeLabelsContext *_localctx = _tracker.createInstance<OC_NodeLabelsContext>(_ctx, getState());
  enterRule(_localctx, 88, CypherParser::RuleOC_NodeLabels);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    size_t alt;
    enterOuterAlt(_localctx, 1);
    setState(832);
    oC_NodeLabel();
    setState(839);
    _errHandler->sync(this);
    alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 137, _ctx);
    while (alt != 2 && alt != atn::ATN::INVALID_ALT_NUMBER) {
      if (alt == 1) {
        setState(834);
        _errHandler->sync(this);

        _la = _input->LA(1);
        if (_la == CypherParser::SP) {
          setState(833);
          match(CypherParser::SP);
        }
        setState(836);
        oC_NodeLabel(); 
      }
      setState(841);
      _errHandler->sync(this);
      alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 137, _ctx);
    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_NodeLabelContext ------------------------------------------------------------------

CypherParser::OC_NodeLabelContext::OC_NodeLabelContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

CypherParser::OC_LabelNameContext* CypherParser::OC_NodeLabelContext::oC_LabelName() {
  return getRuleContext<CypherParser::OC_LabelNameContext>(0);
}

tree::TerminalNode* CypherParser::OC_NodeLabelContext::SP() {
  return getToken(CypherParser::SP, 0);
}


size_t CypherParser::OC_NodeLabelContext::getRuleIndex() const {
  return CypherParser::RuleOC_NodeLabel;
}

void CypherParser::OC_NodeLabelContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_NodeLabel(this);
}

void CypherParser::OC_NodeLabelContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_NodeLabel(this);
}

CypherParser::OC_NodeLabelContext* CypherParser::oC_NodeLabel() {
  OC_NodeLabelContext *_localctx = _tracker.createInstance<OC_NodeLabelContext>(_ctx, getState());
  enterRule(_localctx, 90, CypherParser::RuleOC_NodeLabel);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(842);
    match(CypherParser::T__9);
    setState(844);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (_la == CypherParser::SP) {
      setState(843);
      match(CypherParser::SP);
    }
    setState(846);
    oC_LabelName();
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_RangeLiteralContext ------------------------------------------------------------------

CypherParser::OC_RangeLiteralContext::OC_RangeLiteralContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

std::vector<tree::TerminalNode *> CypherParser::OC_RangeLiteralContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_RangeLiteralContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}

std::vector<CypherParser::OC_IntegerLiteralContext *> CypherParser::OC_RangeLiteralContext::oC_IntegerLiteral() {
  return getRuleContexts<CypherParser::OC_IntegerLiteralContext>();
}

CypherParser::OC_IntegerLiteralContext* CypherParser::OC_RangeLiteralContext::oC_IntegerLiteral(size_t i) {
  return getRuleContext<CypherParser::OC_IntegerLiteralContext>(i);
}


size_t CypherParser::OC_RangeLiteralContext::getRuleIndex() const {
  return CypherParser::RuleOC_RangeLiteral;
}

void CypherParser::OC_RangeLiteralContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_RangeLiteral(this);
}

void CypherParser::OC_RangeLiteralContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_RangeLiteral(this);
}

CypherParser::OC_RangeLiteralContext* CypherParser::oC_RangeLiteral() {
  OC_RangeLiteralContext *_localctx = _tracker.createInstance<OC_RangeLiteralContext>(_ctx, getState());
  enterRule(_localctx, 92, CypherParser::RuleOC_RangeLiteral);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(848);
    match(CypherParser::T__4);
    setState(850);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (_la == CypherParser::SP) {
      setState(849);
      match(CypherParser::SP);
    }
    setState(856);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (((((_la - 97) & ~ 0x3fULL) == 0) &&
      ((1ULL << (_la - 97)) & ((1ULL << (CypherParser::HexInteger - 97))
      | (1ULL << (CypherParser::DecimalInteger - 97))
      | (1ULL << (CypherParser::OctalInteger - 97)))) != 0)) {
      setState(852);
      oC_IntegerLiteral();
      setState(854);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(853);
        match(CypherParser::SP);
      }
    }
    setState(868);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (_la == CypherParser::T__11) {
      setState(858);
      match(CypherParser::T__11);
      setState(860);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(859);
        match(CypherParser::SP);
      }
      setState(866);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (((((_la - 97) & ~ 0x3fULL) == 0) &&
        ((1ULL << (_la - 97)) & ((1ULL << (CypherParser::HexInteger - 97))
        | (1ULL << (CypherParser::DecimalInteger - 97))
        | (1ULL << (CypherParser::OctalInteger - 97)))) != 0)) {
        setState(862);
        oC_IntegerLiteral();
        setState(864);
        _errHandler->sync(this);

        _la = _input->LA(1);
        if (_la == CypherParser::SP) {
          setState(863);
          match(CypherParser::SP);
        }
      }
    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_LabelNameContext ------------------------------------------------------------------

CypherParser::OC_LabelNameContext::OC_LabelNameContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

CypherParser::OC_SchemaNameContext* CypherParser::OC_LabelNameContext::oC_SchemaName() {
  return getRuleContext<CypherParser::OC_SchemaNameContext>(0);
}


size_t CypherParser::OC_LabelNameContext::getRuleIndex() const {
  return CypherParser::RuleOC_LabelName;
}

void CypherParser::OC_LabelNameContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_LabelName(this);
}

void CypherParser::OC_LabelNameContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_LabelName(this);
}

CypherParser::OC_LabelNameContext* CypherParser::oC_LabelName() {
  OC_LabelNameContext *_localctx = _tracker.createInstance<OC_LabelNameContext>(_ctx, getState());
  enterRule(_localctx, 94, CypherParser::RuleOC_LabelName);

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(870);
    oC_SchemaName();
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_RelTypeNameContext ------------------------------------------------------------------

CypherParser::OC_RelTypeNameContext::OC_RelTypeNameContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

CypherParser::OC_SchemaNameContext* CypherParser::OC_RelTypeNameContext::oC_SchemaName() {
  return getRuleContext<CypherParser::OC_SchemaNameContext>(0);
}


size_t CypherParser::OC_RelTypeNameContext::getRuleIndex() const {
  return CypherParser::RuleOC_RelTypeName;
}

void CypherParser::OC_RelTypeNameContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_RelTypeName(this);
}

void CypherParser::OC_RelTypeNameContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_RelTypeName(this);
}

CypherParser::OC_RelTypeNameContext* CypherParser::oC_RelTypeName() {
  OC_RelTypeNameContext *_localctx = _tracker.createInstance<OC_RelTypeNameContext>(_ctx, getState());
  enterRule(_localctx, 96, CypherParser::RuleOC_RelTypeName);

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(872);
    oC_SchemaName();
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_ExpressionContext ------------------------------------------------------------------

CypherParser::OC_ExpressionContext::OC_ExpressionContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

CypherParser::OC_OrExpressionContext* CypherParser::OC_ExpressionContext::oC_OrExpression() {
  return getRuleContext<CypherParser::OC_OrExpressionContext>(0);
}


size_t CypherParser::OC_ExpressionContext::getRuleIndex() const {
  return CypherParser::RuleOC_Expression;
}

void CypherParser::OC_ExpressionContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_Expression(this);
}

void CypherParser::OC_ExpressionContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_Expression(this);
}

CypherParser::OC_ExpressionContext* CypherParser::oC_Expression() {
  OC_ExpressionContext *_localctx = _tracker.createInstance<OC_ExpressionContext>(_ctx, getState());
  enterRule(_localctx, 98, CypherParser::RuleOC_Expression);

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(874);
    oC_OrExpression();
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_OrExpressionContext ------------------------------------------------------------------

CypherParser::OC_OrExpressionContext::OC_OrExpressionContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

std::vector<CypherParser::OC_XorExpressionContext *> CypherParser::OC_OrExpressionContext::oC_XorExpression() {
  return getRuleContexts<CypherParser::OC_XorExpressionContext>();
}

CypherParser::OC_XorExpressionContext* CypherParser::OC_OrExpressionContext::oC_XorExpression(size_t i) {
  return getRuleContext<CypherParser::OC_XorExpressionContext>(i);
}

std::vector<tree::TerminalNode *> CypherParser::OC_OrExpressionContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_OrExpressionContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}

std::vector<tree::TerminalNode *> CypherParser::OC_OrExpressionContext::OR() {
  return getTokens(CypherParser::OR);
}

tree::TerminalNode* CypherParser::OC_OrExpressionContext::OR(size_t i) {
  return getToken(CypherParser::OR, i);
}


size_t CypherParser::OC_OrExpressionContext::getRuleIndex() const {
  return CypherParser::RuleOC_OrExpression;
}

void CypherParser::OC_OrExpressionContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_OrExpression(this);
}

void CypherParser::OC_OrExpressionContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_OrExpression(this);
}

CypherParser::OC_OrExpressionContext* CypherParser::oC_OrExpression() {
  OC_OrExpressionContext *_localctx = _tracker.createInstance<OC_OrExpressionContext>(_ctx, getState());
  enterRule(_localctx, 100, CypherParser::RuleOC_OrExpression);

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    size_t alt;
    enterOuterAlt(_localctx, 1);
    setState(876);
    oC_XorExpression();
    setState(883);
    _errHandler->sync(this);
    alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 146, _ctx);
    while (alt != 2 && alt != atn::ATN::INVALID_ALT_NUMBER) {
      if (alt == 1) {
        setState(877);
        match(CypherParser::SP);
        setState(878);
        match(CypherParser::OR);
        setState(879);
        match(CypherParser::SP);
        setState(880);
        oC_XorExpression(); 
      }
      setState(885);
      _errHandler->sync(this);
      alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 146, _ctx);
    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_XorExpressionContext ------------------------------------------------------------------

CypherParser::OC_XorExpressionContext::OC_XorExpressionContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

std::vector<CypherParser::OC_AndExpressionContext *> CypherParser::OC_XorExpressionContext::oC_AndExpression() {
  return getRuleContexts<CypherParser::OC_AndExpressionContext>();
}

CypherParser::OC_AndExpressionContext* CypherParser::OC_XorExpressionContext::oC_AndExpression(size_t i) {
  return getRuleContext<CypherParser::OC_AndExpressionContext>(i);
}

std::vector<tree::TerminalNode *> CypherParser::OC_XorExpressionContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_XorExpressionContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}

std::vector<tree::TerminalNode *> CypherParser::OC_XorExpressionContext::XOR() {
  return getTokens(CypherParser::XOR);
}

tree::TerminalNode* CypherParser::OC_XorExpressionContext::XOR(size_t i) {
  return getToken(CypherParser::XOR, i);
}


size_t CypherParser::OC_XorExpressionContext::getRuleIndex() const {
  return CypherParser::RuleOC_XorExpression;
}

void CypherParser::OC_XorExpressionContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_XorExpression(this);
}

void CypherParser::OC_XorExpressionContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_XorExpression(this);
}

CypherParser::OC_XorExpressionContext* CypherParser::oC_XorExpression() {
  OC_XorExpressionContext *_localctx = _tracker.createInstance<OC_XorExpressionContext>(_ctx, getState());
  enterRule(_localctx, 102, CypherParser::RuleOC_XorExpression);

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    size_t alt;
    enterOuterAlt(_localctx, 1);
    setState(886);
    oC_AndExpression();
    setState(893);
    _errHandler->sync(this);
    alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 147, _ctx);
    while (alt != 2 && alt != atn::ATN::INVALID_ALT_NUMBER) {
      if (alt == 1) {
        setState(887);
        match(CypherParser::SP);
        setState(888);
        match(CypherParser::XOR);
        setState(889);
        match(CypherParser::SP);
        setState(890);
        oC_AndExpression(); 
      }
      setState(895);
      _errHandler->sync(this);
      alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 147, _ctx);
    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_AndExpressionContext ------------------------------------------------------------------

CypherParser::OC_AndExpressionContext::OC_AndExpressionContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

std::vector<CypherParser::OC_NotExpressionContext *> CypherParser::OC_AndExpressionContext::oC_NotExpression() {
  return getRuleContexts<CypherParser::OC_NotExpressionContext>();
}

CypherParser::OC_NotExpressionContext* CypherParser::OC_AndExpressionContext::oC_NotExpression(size_t i) {
  return getRuleContext<CypherParser::OC_NotExpressionContext>(i);
}

std::vector<tree::TerminalNode *> CypherParser::OC_AndExpressionContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_AndExpressionContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}

std::vector<tree::TerminalNode *> CypherParser::OC_AndExpressionContext::AND() {
  return getTokens(CypherParser::AND);
}

tree::TerminalNode* CypherParser::OC_AndExpressionContext::AND(size_t i) {
  return getToken(CypherParser::AND, i);
}


size_t CypherParser::OC_AndExpressionContext::getRuleIndex() const {
  return CypherParser::RuleOC_AndExpression;
}

void CypherParser::OC_AndExpressionContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_AndExpression(this);
}

void CypherParser::OC_AndExpressionContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_AndExpression(this);
}

CypherParser::OC_AndExpressionContext* CypherParser::oC_AndExpression() {
  OC_AndExpressionContext *_localctx = _tracker.createInstance<OC_AndExpressionContext>(_ctx, getState());
  enterRule(_localctx, 104, CypherParser::RuleOC_AndExpression);

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    size_t alt;
    enterOuterAlt(_localctx, 1);
    setState(896);
    oC_NotExpression();
    setState(903);
    _errHandler->sync(this);
    alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 148, _ctx);
    while (alt != 2 && alt != atn::ATN::INVALID_ALT_NUMBER) {
      if (alt == 1) {
        setState(897);
        match(CypherParser::SP);
        setState(898);
        match(CypherParser::AND);
        setState(899);
        match(CypherParser::SP);
        setState(900);
        oC_NotExpression(); 
      }
      setState(905);
      _errHandler->sync(this);
      alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 148, _ctx);
    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_NotExpressionContext ------------------------------------------------------------------

CypherParser::OC_NotExpressionContext::OC_NotExpressionContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

CypherParser::OC_ComparisonExpressionContext* CypherParser::OC_NotExpressionContext::oC_ComparisonExpression() {
  return getRuleContext<CypherParser::OC_ComparisonExpressionContext>(0);
}

std::vector<tree::TerminalNode *> CypherParser::OC_NotExpressionContext::NOT() {
  return getTokens(CypherParser::NOT);
}

tree::TerminalNode* CypherParser::OC_NotExpressionContext::NOT(size_t i) {
  return getToken(CypherParser::NOT, i);
}

std::vector<tree::TerminalNode *> CypherParser::OC_NotExpressionContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_NotExpressionContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}


size_t CypherParser::OC_NotExpressionContext::getRuleIndex() const {
  return CypherParser::RuleOC_NotExpression;
}

void CypherParser::OC_NotExpressionContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_NotExpression(this);
}

void CypherParser::OC_NotExpressionContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_NotExpression(this);
}

CypherParser::OC_NotExpressionContext* CypherParser::oC_NotExpression() {
  OC_NotExpressionContext *_localctx = _tracker.createInstance<OC_NotExpressionContext>(_ctx, getState());
  enterRule(_localctx, 106, CypherParser::RuleOC_NotExpression);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(912);
    _errHandler->sync(this);
    _la = _input->LA(1);
    while (_la == CypherParser::NOT) {
      setState(906);
      match(CypherParser::NOT);
      setState(908);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(907);
        match(CypherParser::SP);
      }
      setState(914);
      _errHandler->sync(this);
      _la = _input->LA(1);
    }
    setState(915);
    oC_ComparisonExpression();
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_ComparisonExpressionContext ------------------------------------------------------------------

CypherParser::OC_ComparisonExpressionContext::OC_ComparisonExpressionContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

CypherParser::OC_AddOrSubtractExpressionContext* CypherParser::OC_ComparisonExpressionContext::oC_AddOrSubtractExpression() {
  return getRuleContext<CypherParser::OC_AddOrSubtractExpressionContext>(0);
}

std::vector<CypherParser::OC_PartialComparisonExpressionContext *> CypherParser::OC_ComparisonExpressionContext::oC_PartialComparisonExpression() {
  return getRuleContexts<CypherParser::OC_PartialComparisonExpressionContext>();
}

CypherParser::OC_PartialComparisonExpressionContext* CypherParser::OC_ComparisonExpressionContext::oC_PartialComparisonExpression(size_t i) {
  return getRuleContext<CypherParser::OC_PartialComparisonExpressionContext>(i);
}

std::vector<tree::TerminalNode *> CypherParser::OC_ComparisonExpressionContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_ComparisonExpressionContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}


size_t CypherParser::OC_ComparisonExpressionContext::getRuleIndex() const {
  return CypherParser::RuleOC_ComparisonExpression;
}

void CypherParser::OC_ComparisonExpressionContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_ComparisonExpression(this);
}

void CypherParser::OC_ComparisonExpressionContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_ComparisonExpression(this);
}

CypherParser::OC_ComparisonExpressionContext* CypherParser::oC_ComparisonExpression() {
  OC_ComparisonExpressionContext *_localctx = _tracker.createInstance<OC_ComparisonExpressionContext>(_ctx, getState());
  enterRule(_localctx, 108, CypherParser::RuleOC_ComparisonExpression);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    size_t alt;
    enterOuterAlt(_localctx, 1);
    setState(917);
    oC_AddOrSubtractExpression();
    setState(924);
    _errHandler->sync(this);
    alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 152, _ctx);
    while (alt != 2 && alt != atn::ATN::INVALID_ALT_NUMBER) {
      if (alt == 1) {
        setState(919);
        _errHandler->sync(this);

        _la = _input->LA(1);
        if (_la == CypherParser::SP) {
          setState(918);
          match(CypherParser::SP);
        }
        setState(921);
        oC_PartialComparisonExpression(); 
      }
      setState(926);
      _errHandler->sync(this);
      alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 152, _ctx);
    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_AddOrSubtractExpressionContext ------------------------------------------------------------------

CypherParser::OC_AddOrSubtractExpressionContext::OC_AddOrSubtractExpressionContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

std::vector<CypherParser::OC_MultiplyDivideModuloExpressionContext *> CypherParser::OC_AddOrSubtractExpressionContext::oC_MultiplyDivideModuloExpression() {
  return getRuleContexts<CypherParser::OC_MultiplyDivideModuloExpressionContext>();
}

CypherParser::OC_MultiplyDivideModuloExpressionContext* CypherParser::OC_AddOrSubtractExpressionContext::oC_MultiplyDivideModuloExpression(size_t i) {
  return getRuleContext<CypherParser::OC_MultiplyDivideModuloExpressionContext>(i);
}

std::vector<tree::TerminalNode *> CypherParser::OC_AddOrSubtractExpressionContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_AddOrSubtractExpressionContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}


size_t CypherParser::OC_AddOrSubtractExpressionContext::getRuleIndex() const {
  return CypherParser::RuleOC_AddOrSubtractExpression;
}

void CypherParser::OC_AddOrSubtractExpressionContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_AddOrSubtractExpression(this);
}

void CypherParser::OC_AddOrSubtractExpressionContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_AddOrSubtractExpression(this);
}

CypherParser::OC_AddOrSubtractExpressionContext* CypherParser::oC_AddOrSubtractExpression() {
  OC_AddOrSubtractExpressionContext *_localctx = _tracker.createInstance<OC_AddOrSubtractExpressionContext>(_ctx, getState());
  enterRule(_localctx, 110, CypherParser::RuleOC_AddOrSubtractExpression);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    size_t alt;
    enterOuterAlt(_localctx, 1);
    setState(927);
    oC_MultiplyDivideModuloExpression();
    setState(946);
    _errHandler->sync(this);
    alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 158, _ctx);
    while (alt != 2 && alt != atn::ATN::INVALID_ALT_NUMBER) {
      if (alt == 1) {
        setState(944);
        _errHandler->sync(this);
        switch (getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 157, _ctx)) {
        case 1: {
          setState(929);
          _errHandler->sync(this);

          _la = _input->LA(1);
          if (_la == CypherParser::SP) {
            setState(928);
            match(CypherParser::SP);
          }
          setState(931);
          match(CypherParser::T__12);
          setState(933);
          _errHandler->sync(this);

          _la = _input->LA(1);
          if (_la == CypherParser::SP) {
            setState(932);
            match(CypherParser::SP);
          }
          setState(935);
          oC_MultiplyDivideModuloExpression();
          break;
        }

        case 2: {
          setState(937);
          _errHandler->sync(this);

          _la = _input->LA(1);
          if (_la == CypherParser::SP) {
            setState(936);
            match(CypherParser::SP);
          }
          setState(939);
          match(CypherParser::T__13);
          setState(941);
          _errHandler->sync(this);

          _la = _input->LA(1);
          if (_la == CypherParser::SP) {
            setState(940);
            match(CypherParser::SP);
          }
          setState(943);
          oC_MultiplyDivideModuloExpression();
          break;
        }

        } 
      }
      setState(948);
      _errHandler->sync(this);
      alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 158, _ctx);
    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_MultiplyDivideModuloExpressionContext ------------------------------------------------------------------

CypherParser::OC_MultiplyDivideModuloExpressionContext::OC_MultiplyDivideModuloExpressionContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

std::vector<CypherParser::OC_PowerOfExpressionContext *> CypherParser::OC_MultiplyDivideModuloExpressionContext::oC_PowerOfExpression() {
  return getRuleContexts<CypherParser::OC_PowerOfExpressionContext>();
}

CypherParser::OC_PowerOfExpressionContext* CypherParser::OC_MultiplyDivideModuloExpressionContext::oC_PowerOfExpression(size_t i) {
  return getRuleContext<CypherParser::OC_PowerOfExpressionContext>(i);
}

std::vector<tree::TerminalNode *> CypherParser::OC_MultiplyDivideModuloExpressionContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_MultiplyDivideModuloExpressionContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}


size_t CypherParser::OC_MultiplyDivideModuloExpressionContext::getRuleIndex() const {
  return CypherParser::RuleOC_MultiplyDivideModuloExpression;
}

void CypherParser::OC_MultiplyDivideModuloExpressionContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_MultiplyDivideModuloExpression(this);
}

void CypherParser::OC_MultiplyDivideModuloExpressionContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_MultiplyDivideModuloExpression(this);
}

CypherParser::OC_MultiplyDivideModuloExpressionContext* CypherParser::oC_MultiplyDivideModuloExpression() {
  OC_MultiplyDivideModuloExpressionContext *_localctx = _tracker.createInstance<OC_MultiplyDivideModuloExpressionContext>(_ctx, getState());
  enterRule(_localctx, 112, CypherParser::RuleOC_MultiplyDivideModuloExpression);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    size_t alt;
    enterOuterAlt(_localctx, 1);
    setState(949);
    oC_PowerOfExpression();
    setState(976);
    _errHandler->sync(this);
    alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 166, _ctx);
    while (alt != 2 && alt != atn::ATN::INVALID_ALT_NUMBER) {
      if (alt == 1) {
        setState(974);
        _errHandler->sync(this);
        switch (getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 165, _ctx)) {
        case 1: {
          setState(951);
          _errHandler->sync(this);

          _la = _input->LA(1);
          if (_la == CypherParser::SP) {
            setState(950);
            match(CypherParser::SP);
          }
          setState(953);
          match(CypherParser::T__4);
          setState(955);
          _errHandler->sync(this);

          _la = _input->LA(1);
          if (_la == CypherParser::SP) {
            setState(954);
            match(CypherParser::SP);
          }
          setState(957);
          oC_PowerOfExpression();
          break;
        }

        case 2: {
          setState(959);
          _errHandler->sync(this);

          _la = _input->LA(1);
          if (_la == CypherParser::SP) {
            setState(958);
            match(CypherParser::SP);
          }
          setState(961);
          match(CypherParser::T__14);
          setState(963);
          _errHandler->sync(this);

          _la = _input->LA(1);
          if (_la == CypherParser::SP) {
            setState(962);
            match(CypherParser::SP);
          }
          setState(965);
          oC_PowerOfExpression();
          break;
        }

        case 3: {
          setState(967);
          _errHandler->sync(this);

          _la = _input->LA(1);
          if (_la == CypherParser::SP) {
            setState(966);
            match(CypherParser::SP);
          }
          setState(969);
          match(CypherParser::T__15);
          setState(971);
          _errHandler->sync(this);

          _la = _input->LA(1);
          if (_la == CypherParser::SP) {
            setState(970);
            match(CypherParser::SP);
          }
          setState(973);
          oC_PowerOfExpression();
          break;
        }

        } 
      }
      setState(978);
      _errHandler->sync(this);
      alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 166, _ctx);
    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_PowerOfExpressionContext ------------------------------------------------------------------

CypherParser::OC_PowerOfExpressionContext::OC_PowerOfExpressionContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

std::vector<CypherParser::OC_UnaryAddOrSubtractExpressionContext *> CypherParser::OC_PowerOfExpressionContext::oC_UnaryAddOrSubtractExpression() {
  return getRuleContexts<CypherParser::OC_UnaryAddOrSubtractExpressionContext>();
}

CypherParser::OC_UnaryAddOrSubtractExpressionContext* CypherParser::OC_PowerOfExpressionContext::oC_UnaryAddOrSubtractExpression(size_t i) {
  return getRuleContext<CypherParser::OC_UnaryAddOrSubtractExpressionContext>(i);
}

std::vector<tree::TerminalNode *> CypherParser::OC_PowerOfExpressionContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_PowerOfExpressionContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}


size_t CypherParser::OC_PowerOfExpressionContext::getRuleIndex() const {
  return CypherParser::RuleOC_PowerOfExpression;
}

void CypherParser::OC_PowerOfExpressionContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_PowerOfExpression(this);
}

void CypherParser::OC_PowerOfExpressionContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_PowerOfExpression(this);
}

CypherParser::OC_PowerOfExpressionContext* CypherParser::oC_PowerOfExpression() {
  OC_PowerOfExpressionContext *_localctx = _tracker.createInstance<OC_PowerOfExpressionContext>(_ctx, getState());
  enterRule(_localctx, 114, CypherParser::RuleOC_PowerOfExpression);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    size_t alt;
    enterOuterAlt(_localctx, 1);
    setState(979);
    oC_UnaryAddOrSubtractExpression();
    setState(990);
    _errHandler->sync(this);
    alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 169, _ctx);
    while (alt != 2 && alt != atn::ATN::INVALID_ALT_NUMBER) {
      if (alt == 1) {
        setState(981);
        _errHandler->sync(this);

        _la = _input->LA(1);
        if (_la == CypherParser::SP) {
          setState(980);
          match(CypherParser::SP);
        }
        setState(983);
        match(CypherParser::T__16);
        setState(985);
        _errHandler->sync(this);

        _la = _input->LA(1);
        if (_la == CypherParser::SP) {
          setState(984);
          match(CypherParser::SP);
        }
        setState(987);
        oC_UnaryAddOrSubtractExpression(); 
      }
      setState(992);
      _errHandler->sync(this);
      alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 169, _ctx);
    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_UnaryAddOrSubtractExpressionContext ------------------------------------------------------------------

CypherParser::OC_UnaryAddOrSubtractExpressionContext::OC_UnaryAddOrSubtractExpressionContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

CypherParser::OC_StringListNullOperatorExpressionContext* CypherParser::OC_UnaryAddOrSubtractExpressionContext::oC_StringListNullOperatorExpression() {
  return getRuleContext<CypherParser::OC_StringListNullOperatorExpressionContext>(0);
}

std::vector<tree::TerminalNode *> CypherParser::OC_UnaryAddOrSubtractExpressionContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_UnaryAddOrSubtractExpressionContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}


size_t CypherParser::OC_UnaryAddOrSubtractExpressionContext::getRuleIndex() const {
  return CypherParser::RuleOC_UnaryAddOrSubtractExpression;
}

void CypherParser::OC_UnaryAddOrSubtractExpressionContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_UnaryAddOrSubtractExpression(this);
}

void CypherParser::OC_UnaryAddOrSubtractExpressionContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_UnaryAddOrSubtractExpression(this);
}

CypherParser::OC_UnaryAddOrSubtractExpressionContext* CypherParser::oC_UnaryAddOrSubtractExpression() {
  OC_UnaryAddOrSubtractExpressionContext *_localctx = _tracker.createInstance<OC_UnaryAddOrSubtractExpressionContext>(_ctx, getState());
  enterRule(_localctx, 116, CypherParser::RuleOC_UnaryAddOrSubtractExpression);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(999);
    _errHandler->sync(this);
    _la = _input->LA(1);
    while (_la == CypherParser::T__12

    || _la == CypherParser::T__13) {
      setState(993);
      _la = _input->LA(1);
      if (!(_la == CypherParser::T__12

      || _la == CypherParser::T__13)) {
      _errHandler->recoverInline(this);
      }
      else {
        _errHandler->reportMatch(this);
        consume();
      }
      setState(995);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(994);
        match(CypherParser::SP);
      }
      setState(1001);
      _errHandler->sync(this);
      _la = _input->LA(1);
    }
    setState(1002);
    oC_StringListNullOperatorExpression();
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_StringListNullOperatorExpressionContext ------------------------------------------------------------------

CypherParser::OC_StringListNullOperatorExpressionContext::OC_StringListNullOperatorExpressionContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

CypherParser::OC_PropertyOrLabelsExpressionContext* CypherParser::OC_StringListNullOperatorExpressionContext::oC_PropertyOrLabelsExpression() {
  return getRuleContext<CypherParser::OC_PropertyOrLabelsExpressionContext>(0);
}

std::vector<CypherParser::OC_StringOperatorExpressionContext *> CypherParser::OC_StringListNullOperatorExpressionContext::oC_StringOperatorExpression() {
  return getRuleContexts<CypherParser::OC_StringOperatorExpressionContext>();
}

CypherParser::OC_StringOperatorExpressionContext* CypherParser::OC_StringListNullOperatorExpressionContext::oC_StringOperatorExpression(size_t i) {
  return getRuleContext<CypherParser::OC_StringOperatorExpressionContext>(i);
}

std::vector<CypherParser::OC_ListOperatorExpressionContext *> CypherParser::OC_StringListNullOperatorExpressionContext::oC_ListOperatorExpression() {
  return getRuleContexts<CypherParser::OC_ListOperatorExpressionContext>();
}

CypherParser::OC_ListOperatorExpressionContext* CypherParser::OC_StringListNullOperatorExpressionContext::oC_ListOperatorExpression(size_t i) {
  return getRuleContext<CypherParser::OC_ListOperatorExpressionContext>(i);
}

std::vector<CypherParser::OC_NullOperatorExpressionContext *> CypherParser::OC_StringListNullOperatorExpressionContext::oC_NullOperatorExpression() {
  return getRuleContexts<CypherParser::OC_NullOperatorExpressionContext>();
}

CypherParser::OC_NullOperatorExpressionContext* CypherParser::OC_StringListNullOperatorExpressionContext::oC_NullOperatorExpression(size_t i) {
  return getRuleContext<CypherParser::OC_NullOperatorExpressionContext>(i);
}


size_t CypherParser::OC_StringListNullOperatorExpressionContext::getRuleIndex() const {
  return CypherParser::RuleOC_StringListNullOperatorExpression;
}

void CypherParser::OC_StringListNullOperatorExpressionContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_StringListNullOperatorExpression(this);
}

void CypherParser::OC_StringListNullOperatorExpressionContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_StringListNullOperatorExpression(this);
}

CypherParser::OC_StringListNullOperatorExpressionContext* CypherParser::oC_StringListNullOperatorExpression() {
  OC_StringListNullOperatorExpressionContext *_localctx = _tracker.createInstance<OC_StringListNullOperatorExpressionContext>(_ctx, getState());
  enterRule(_localctx, 118, CypherParser::RuleOC_StringListNullOperatorExpression);

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    size_t alt;
    enterOuterAlt(_localctx, 1);
    setState(1004);
    oC_PropertyOrLabelsExpression();
    setState(1010);
    _errHandler->sync(this);
    alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 173, _ctx);
    while (alt != 2 && alt != atn::ATN::INVALID_ALT_NUMBER) {
      if (alt == 1) {
        setState(1008);
        _errHandler->sync(this);
        switch (getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 172, _ctx)) {
        case 1: {
          setState(1005);
          oC_StringOperatorExpression();
          break;
        }

        case 2: {
          setState(1006);
          oC_ListOperatorExpression();
          break;
        }

        case 3: {
          setState(1007);
          oC_NullOperatorExpression();
          break;
        }

        } 
      }
      setState(1012);
      _errHandler->sync(this);
      alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 173, _ctx);
    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_ListOperatorExpressionContext ------------------------------------------------------------------

CypherParser::OC_ListOperatorExpressionContext::OC_ListOperatorExpressionContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

std::vector<tree::TerminalNode *> CypherParser::OC_ListOperatorExpressionContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_ListOperatorExpressionContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}

tree::TerminalNode* CypherParser::OC_ListOperatorExpressionContext::IN() {
  return getToken(CypherParser::IN, 0);
}

CypherParser::OC_PropertyOrLabelsExpressionContext* CypherParser::OC_ListOperatorExpressionContext::oC_PropertyOrLabelsExpression() {
  return getRuleContext<CypherParser::OC_PropertyOrLabelsExpressionContext>(0);
}

std::vector<CypherParser::OC_ExpressionContext *> CypherParser::OC_ListOperatorExpressionContext::oC_Expression() {
  return getRuleContexts<CypherParser::OC_ExpressionContext>();
}

CypherParser::OC_ExpressionContext* CypherParser::OC_ListOperatorExpressionContext::oC_Expression(size_t i) {
  return getRuleContext<CypherParser::OC_ExpressionContext>(i);
}


size_t CypherParser::OC_ListOperatorExpressionContext::getRuleIndex() const {
  return CypherParser::RuleOC_ListOperatorExpression;
}

void CypherParser::OC_ListOperatorExpressionContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_ListOperatorExpression(this);
}

void CypherParser::OC_ListOperatorExpressionContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_ListOperatorExpression(this);
}

CypherParser::OC_ListOperatorExpressionContext* CypherParser::oC_ListOperatorExpression() {
  OC_ListOperatorExpressionContext *_localctx = _tracker.createInstance<OC_ListOperatorExpressionContext>(_ctx, getState());
  enterRule(_localctx, 120, CypherParser::RuleOC_ListOperatorExpression);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    setState(1038);
    _errHandler->sync(this);
    switch (getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 179, _ctx)) {
    case 1: {
      enterOuterAlt(_localctx, 1);
      setState(1013);
      match(CypherParser::SP);
      setState(1014);
      match(CypherParser::IN);
      setState(1016);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(1015);
        match(CypherParser::SP);
      }
      setState(1018);
      oC_PropertyOrLabelsExpression();
      break;
    }

    case 2: {
      enterOuterAlt(_localctx, 2);
      setState(1020);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(1019);
        match(CypherParser::SP);
      }
      setState(1022);
      match(CypherParser::T__7);
      setState(1023);
      oC_Expression();
      setState(1024);
      match(CypherParser::T__8);
      break;
    }

    case 3: {
      enterOuterAlt(_localctx, 3);
      setState(1027);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(1026);
        match(CypherParser::SP);
      }
      setState(1029);
      match(CypherParser::T__7);
      setState(1031);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if ((((_la & ~ 0x3fULL) == 0) &&
        ((1ULL << _la) & ((1ULL << CypherParser::T__5)
        | (1ULL << CypherParser::T__7)
        | (1ULL << CypherParser::T__12)
        | (1ULL << CypherParser::T__13)
        | (1ULL << CypherParser::T__23)
        | (1ULL << CypherParser::T__25)
        | (1ULL << CypherParser::ALL))) != 0) || ((((_la - 76) & ~ 0x3fULL) == 0) &&
        ((1ULL << (_la - 76)) & ((1ULL << (CypherParser::NOT - 76))
        | (1ULL << (CypherParser::NULLOP - 76))
        | (1ULL << (CypherParser::COUNT - 76))
        | (1ULL << (CypherParser::ANY - 76))
        | (1ULL << (CypherParser::NONE - 76))
        | (1ULL << (CypherParser::SINGLE - 76))
        | (1ULL << (CypherParser::TRUE - 76))
        | (1ULL << (CypherParser::FALSE - 76))
        | (1ULL << (CypherParser::EXISTS - 76))
        | (1ULL << (CypherParser::CASE - 76))
        | (1ULL << (CypherParser::StringLiteral - 76))
        | (1ULL << (CypherParser::HexInteger - 76))
        | (1ULL << (CypherParser::DecimalInteger - 76))
        | (1ULL << (CypherParser::OctalInteger - 76))
        | (1ULL << (CypherParser::HexLetter - 76))
        | (1ULL << (CypherParser::ExponentDecimalReal - 76))
        | (1ULL << (CypherParser::RegularDecimalReal - 76))
        | (1ULL << (CypherParser::FILTER - 76))
        | (1ULL << (CypherParser::EXTRACT - 76))
        | (1ULL << (CypherParser::UnescapedSymbolicName - 76))
        | (1ULL << (CypherParser::EscapedSymbolicName - 76)))) != 0)) {
        setState(1030);
        oC_Expression();
      }
      setState(1033);
      match(CypherParser::T__11);
      setState(1035);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if ((((_la & ~ 0x3fULL) == 0) &&
        ((1ULL << _la) & ((1ULL << CypherParser::T__5)
        | (1ULL << CypherParser::T__7)
        | (1ULL << CypherParser::T__12)
        | (1ULL << CypherParser::T__13)
        | (1ULL << CypherParser::T__23)
        | (1ULL << CypherParser::T__25)
        | (1ULL << CypherParser::ALL))) != 0) || ((((_la - 76) & ~ 0x3fULL) == 0) &&
        ((1ULL << (_la - 76)) & ((1ULL << (CypherParser::NOT - 76))
        | (1ULL << (CypherParser::NULLOP - 76))
        | (1ULL << (CypherParser::COUNT - 76))
        | (1ULL << (CypherParser::ANY - 76))
        | (1ULL << (CypherParser::NONE - 76))
        | (1ULL << (CypherParser::SINGLE - 76))
        | (1ULL << (CypherParser::TRUE - 76))
        | (1ULL << (CypherParser::FALSE - 76))
        | (1ULL << (CypherParser::EXISTS - 76))
        | (1ULL << (CypherParser::CASE - 76))
        | (1ULL << (CypherParser::StringLiteral - 76))
        | (1ULL << (CypherParser::HexInteger - 76))
        | (1ULL << (CypherParser::DecimalInteger - 76))
        | (1ULL << (CypherParser::OctalInteger - 76))
        | (1ULL << (CypherParser::HexLetter - 76))
        | (1ULL << (CypherParser::ExponentDecimalReal - 76))
        | (1ULL << (CypherParser::RegularDecimalReal - 76))
        | (1ULL << (CypherParser::FILTER - 76))
        | (1ULL << (CypherParser::EXTRACT - 76))
        | (1ULL << (CypherParser::UnescapedSymbolicName - 76))
        | (1ULL << (CypherParser::EscapedSymbolicName - 76)))) != 0)) {
        setState(1034);
        oC_Expression();
      }
      setState(1037);
      match(CypherParser::T__8);
      break;
    }

    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_StringOperatorExpressionContext ------------------------------------------------------------------

CypherParser::OC_StringOperatorExpressionContext::OC_StringOperatorExpressionContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

CypherParser::OC_PropertyOrLabelsExpressionContext* CypherParser::OC_StringOperatorExpressionContext::oC_PropertyOrLabelsExpression() {
  return getRuleContext<CypherParser::OC_PropertyOrLabelsExpressionContext>(0);
}

std::vector<tree::TerminalNode *> CypherParser::OC_StringOperatorExpressionContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_StringOperatorExpressionContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}

tree::TerminalNode* CypherParser::OC_StringOperatorExpressionContext::STARTS() {
  return getToken(CypherParser::STARTS, 0);
}

tree::TerminalNode* CypherParser::OC_StringOperatorExpressionContext::WITH() {
  return getToken(CypherParser::WITH, 0);
}

tree::TerminalNode* CypherParser::OC_StringOperatorExpressionContext::ENDS() {
  return getToken(CypherParser::ENDS, 0);
}

tree::TerminalNode* CypherParser::OC_StringOperatorExpressionContext::CONTAINS() {
  return getToken(CypherParser::CONTAINS, 0);
}


size_t CypherParser::OC_StringOperatorExpressionContext::getRuleIndex() const {
  return CypherParser::RuleOC_StringOperatorExpression;
}

void CypherParser::OC_StringOperatorExpressionContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_StringOperatorExpression(this);
}

void CypherParser::OC_StringOperatorExpressionContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_StringOperatorExpression(this);
}

CypherParser::OC_StringOperatorExpressionContext* CypherParser::oC_StringOperatorExpression() {
  OC_StringOperatorExpressionContext *_localctx = _tracker.createInstance<OC_StringOperatorExpressionContext>(_ctx, getState());
  enterRule(_localctx, 122, CypherParser::RuleOC_StringOperatorExpression);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(1050);
    _errHandler->sync(this);
    switch (getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 180, _ctx)) {
    case 1: {
      setState(1040);
      match(CypherParser::SP);
      setState(1041);
      match(CypherParser::STARTS);
      setState(1042);
      match(CypherParser::SP);
      setState(1043);
      match(CypherParser::WITH);
      break;
    }

    case 2: {
      setState(1044);
      match(CypherParser::SP);
      setState(1045);
      match(CypherParser::ENDS);
      setState(1046);
      match(CypherParser::SP);
      setState(1047);
      match(CypherParser::WITH);
      break;
    }

    case 3: {
      setState(1048);
      match(CypherParser::SP);
      setState(1049);
      match(CypherParser::CONTAINS);
      break;
    }

    }
    setState(1053);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (_la == CypherParser::SP) {
      setState(1052);
      match(CypherParser::SP);
    }
    setState(1055);
    oC_PropertyOrLabelsExpression();
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_NullOperatorExpressionContext ------------------------------------------------------------------

CypherParser::OC_NullOperatorExpressionContext::OC_NullOperatorExpressionContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

std::vector<tree::TerminalNode *> CypherParser::OC_NullOperatorExpressionContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_NullOperatorExpressionContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}

tree::TerminalNode* CypherParser::OC_NullOperatorExpressionContext::IS() {
  return getToken(CypherParser::IS, 0);
}

tree::TerminalNode* CypherParser::OC_NullOperatorExpressionContext::NULLOP() {
  return getToken(CypherParser::NULLOP, 0);
}

tree::TerminalNode* CypherParser::OC_NullOperatorExpressionContext::NOT() {
  return getToken(CypherParser::NOT, 0);
}


size_t CypherParser::OC_NullOperatorExpressionContext::getRuleIndex() const {
  return CypherParser::RuleOC_NullOperatorExpression;
}

void CypherParser::OC_NullOperatorExpressionContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_NullOperatorExpression(this);
}

void CypherParser::OC_NullOperatorExpressionContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_NullOperatorExpression(this);
}

CypherParser::OC_NullOperatorExpressionContext* CypherParser::oC_NullOperatorExpression() {
  OC_NullOperatorExpressionContext *_localctx = _tracker.createInstance<OC_NullOperatorExpressionContext>(_ctx, getState());
  enterRule(_localctx, 124, CypherParser::RuleOC_NullOperatorExpression);

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    setState(1067);
    _errHandler->sync(this);
    switch (getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 182, _ctx)) {
    case 1: {
      enterOuterAlt(_localctx, 1);
      setState(1057);
      match(CypherParser::SP);
      setState(1058);
      match(CypherParser::IS);
      setState(1059);
      match(CypherParser::SP);
      setState(1060);
      match(CypherParser::NULLOP);
      break;
    }

    case 2: {
      enterOuterAlt(_localctx, 2);
      setState(1061);
      match(CypherParser::SP);
      setState(1062);
      match(CypherParser::IS);
      setState(1063);
      match(CypherParser::SP);
      setState(1064);
      match(CypherParser::NOT);
      setState(1065);
      match(CypherParser::SP);
      setState(1066);
      match(CypherParser::NULLOP);
      break;
    }

    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_PropertyOrLabelsExpressionContext ------------------------------------------------------------------

CypherParser::OC_PropertyOrLabelsExpressionContext::OC_PropertyOrLabelsExpressionContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

CypherParser::OC_AtomContext* CypherParser::OC_PropertyOrLabelsExpressionContext::oC_Atom() {
  return getRuleContext<CypherParser::OC_AtomContext>(0);
}

std::vector<CypherParser::OC_PropertyLookupContext *> CypherParser::OC_PropertyOrLabelsExpressionContext::oC_PropertyLookup() {
  return getRuleContexts<CypherParser::OC_PropertyLookupContext>();
}

CypherParser::OC_PropertyLookupContext* CypherParser::OC_PropertyOrLabelsExpressionContext::oC_PropertyLookup(size_t i) {
  return getRuleContext<CypherParser::OC_PropertyLookupContext>(i);
}

CypherParser::OC_NodeLabelsContext* CypherParser::OC_PropertyOrLabelsExpressionContext::oC_NodeLabels() {
  return getRuleContext<CypherParser::OC_NodeLabelsContext>(0);
}

std::vector<tree::TerminalNode *> CypherParser::OC_PropertyOrLabelsExpressionContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_PropertyOrLabelsExpressionContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}


size_t CypherParser::OC_PropertyOrLabelsExpressionContext::getRuleIndex() const {
  return CypherParser::RuleOC_PropertyOrLabelsExpression;
}

void CypherParser::OC_PropertyOrLabelsExpressionContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_PropertyOrLabelsExpression(this);
}

void CypherParser::OC_PropertyOrLabelsExpressionContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_PropertyOrLabelsExpression(this);
}

CypherParser::OC_PropertyOrLabelsExpressionContext* CypherParser::oC_PropertyOrLabelsExpression() {
  OC_PropertyOrLabelsExpressionContext *_localctx = _tracker.createInstance<OC_PropertyOrLabelsExpressionContext>(_ctx, getState());
  enterRule(_localctx, 126, CypherParser::RuleOC_PropertyOrLabelsExpression);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    size_t alt;
    enterOuterAlt(_localctx, 1);
    setState(1069);
    oC_Atom();
    setState(1076);
    _errHandler->sync(this);
    alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 184, _ctx);
    while (alt != 2 && alt != atn::ATN::INVALID_ALT_NUMBER) {
      if (alt == 1) {
        setState(1071);
        _errHandler->sync(this);

        _la = _input->LA(1);
        if (_la == CypherParser::SP) {
          setState(1070);
          match(CypherParser::SP);
        }
        setState(1073);
        oC_PropertyLookup(); 
      }
      setState(1078);
      _errHandler->sync(this);
      alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 184, _ctx);
    }
    setState(1083);
    _errHandler->sync(this);

    switch (getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 186, _ctx)) {
    case 1: {
      setState(1080);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(1079);
        match(CypherParser::SP);
      }
      setState(1082);
      oC_NodeLabels();
      break;
    }

    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_AtomContext ------------------------------------------------------------------

CypherParser::OC_AtomContext::OC_AtomContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

CypherParser::OC_LiteralContext* CypherParser::OC_AtomContext::oC_Literal() {
  return getRuleContext<CypherParser::OC_LiteralContext>(0);
}

CypherParser::OC_ParameterContext* CypherParser::OC_AtomContext::oC_Parameter() {
  return getRuleContext<CypherParser::OC_ParameterContext>(0);
}

CypherParser::OC_CaseExpressionContext* CypherParser::OC_AtomContext::oC_CaseExpression() {
  return getRuleContext<CypherParser::OC_CaseExpressionContext>(0);
}

tree::TerminalNode* CypherParser::OC_AtomContext::COUNT() {
  return getToken(CypherParser::COUNT, 0);
}

std::vector<tree::TerminalNode *> CypherParser::OC_AtomContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_AtomContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}

CypherParser::OC_ListComprehensionContext* CypherParser::OC_AtomContext::oC_ListComprehension() {
  return getRuleContext<CypherParser::OC_ListComprehensionContext>(0);
}

CypherParser::OC_PatternComprehensionContext* CypherParser::OC_AtomContext::oC_PatternComprehension() {
  return getRuleContext<CypherParser::OC_PatternComprehensionContext>(0);
}

tree::TerminalNode* CypherParser::OC_AtomContext::ALL() {
  return getToken(CypherParser::ALL, 0);
}

CypherParser::OC_FilterExpressionContext* CypherParser::OC_AtomContext::oC_FilterExpression() {
  return getRuleContext<CypherParser::OC_FilterExpressionContext>(0);
}

tree::TerminalNode* CypherParser::OC_AtomContext::ANY() {
  return getToken(CypherParser::ANY, 0);
}

tree::TerminalNode* CypherParser::OC_AtomContext::NONE() {
  return getToken(CypherParser::NONE, 0);
}

tree::TerminalNode* CypherParser::OC_AtomContext::SINGLE() {
  return getToken(CypherParser::SINGLE, 0);
}

CypherParser::OC_RelationshipsPatternContext* CypherParser::OC_AtomContext::oC_RelationshipsPattern() {
  return getRuleContext<CypherParser::OC_RelationshipsPatternContext>(0);
}

CypherParser::OC_ParenthesizedExpressionContext* CypherParser::OC_AtomContext::oC_ParenthesizedExpression() {
  return getRuleContext<CypherParser::OC_ParenthesizedExpressionContext>(0);
}

CypherParser::OC_FunctionInvocationContext* CypherParser::OC_AtomContext::oC_FunctionInvocation() {
  return getRuleContext<CypherParser::OC_FunctionInvocationContext>(0);
}

CypherParser::OC_VariableContext* CypherParser::OC_AtomContext::oC_Variable() {
  return getRuleContext<CypherParser::OC_VariableContext>(0);
}


size_t CypherParser::OC_AtomContext::getRuleIndex() const {
  return CypherParser::RuleOC_Atom;
}

void CypherParser::OC_AtomContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_Atom(this);
}

void CypherParser::OC_AtomContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_Atom(this);
}

CypherParser::OC_AtomContext* CypherParser::oC_Atom() {
  OC_AtomContext *_localctx = _tracker.createInstance<OC_AtomContext>(_ctx, getState());
  enterRule(_localctx, 128, CypherParser::RuleOC_Atom);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    setState(1163);
    _errHandler->sync(this);
    switch (getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 202, _ctx)) {
    case 1: {
      enterOuterAlt(_localctx, 1);
      setState(1085);
      oC_Literal();
      break;
    }

    case 2: {
      enterOuterAlt(_localctx, 2);
      setState(1086);
      oC_Parameter();
      break;
    }

    case 3: {
      enterOuterAlt(_localctx, 3);
      setState(1087);
      oC_CaseExpression();
      break;
    }

    case 4: {
      enterOuterAlt(_localctx, 4);
      setState(1088);
      match(CypherParser::COUNT);
      setState(1090);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(1089);
        match(CypherParser::SP);
      }
      setState(1092);
      match(CypherParser::T__5);
      setState(1094);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(1093);
        match(CypherParser::SP);
      }
      setState(1096);
      match(CypherParser::T__4);
      setState(1098);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(1097);
        match(CypherParser::SP);
      }
      setState(1100);
      match(CypherParser::T__6);
      break;
    }

    case 5: {
      enterOuterAlt(_localctx, 5);
      setState(1101);
      oC_ListComprehension();
      break;
    }

    case 6: {
      enterOuterAlt(_localctx, 6);
      setState(1102);
      oC_PatternComprehension();
      break;
    }

    case 7: {
      enterOuterAlt(_localctx, 7);
      setState(1103);
      match(CypherParser::ALL);
      setState(1105);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(1104);
        match(CypherParser::SP);
      }
      setState(1107);
      match(CypherParser::T__5);
      setState(1109);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(1108);
        match(CypherParser::SP);
      }
      setState(1111);
      oC_FilterExpression();
      setState(1113);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(1112);
        match(CypherParser::SP);
      }
      setState(1115);
      match(CypherParser::T__6);
      break;
    }

    case 8: {
      enterOuterAlt(_localctx, 8);
      setState(1117);
      match(CypherParser::ANY);
      setState(1119);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(1118);
        match(CypherParser::SP);
      }
      setState(1121);
      match(CypherParser::T__5);
      setState(1123);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(1122);
        match(CypherParser::SP);
      }
      setState(1125);
      oC_FilterExpression();
      setState(1127);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(1126);
        match(CypherParser::SP);
      }
      setState(1129);
      match(CypherParser::T__6);
      break;
    }

    case 9: {
      enterOuterAlt(_localctx, 9);
      setState(1131);
      match(CypherParser::NONE);
      setState(1133);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(1132);
        match(CypherParser::SP);
      }
      setState(1135);
      match(CypherParser::T__5);
      setState(1137);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(1136);
        match(CypherParser::SP);
      }
      setState(1139);
      oC_FilterExpression();
      setState(1141);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(1140);
        match(CypherParser::SP);
      }
      setState(1143);
      match(CypherParser::T__6);
      break;
    }

    case 10: {
      enterOuterAlt(_localctx, 10);
      setState(1145);
      match(CypherParser::SINGLE);
      setState(1147);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(1146);
        match(CypherParser::SP);
      }
      setState(1149);
      match(CypherParser::T__5);
      setState(1151);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(1150);
        match(CypherParser::SP);
      }
      setState(1153);
      oC_FilterExpression();
      setState(1155);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(1154);
        match(CypherParser::SP);
      }
      setState(1157);
      match(CypherParser::T__6);
      break;
    }

    case 11: {
      enterOuterAlt(_localctx, 11);
      setState(1159);
      oC_RelationshipsPattern();
      break;
    }

    case 12: {
      enterOuterAlt(_localctx, 12);
      setState(1160);
      oC_ParenthesizedExpression();
      break;
    }

    case 13: {
      enterOuterAlt(_localctx, 13);
      setState(1161);
      oC_FunctionInvocation();
      break;
    }

    case 14: {
      enterOuterAlt(_localctx, 14);
      setState(1162);
      oC_Variable();
      break;
    }

    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_LiteralContext ------------------------------------------------------------------

CypherParser::OC_LiteralContext::OC_LiteralContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

CypherParser::OC_NumberLiteralContext* CypherParser::OC_LiteralContext::oC_NumberLiteral() {
  return getRuleContext<CypherParser::OC_NumberLiteralContext>(0);
}

tree::TerminalNode* CypherParser::OC_LiteralContext::StringLiteral() {
  return getToken(CypherParser::StringLiteral, 0);
}

CypherParser::OC_BooleanLiteralContext* CypherParser::OC_LiteralContext::oC_BooleanLiteral() {
  return getRuleContext<CypherParser::OC_BooleanLiteralContext>(0);
}

tree::TerminalNode* CypherParser::OC_LiteralContext::NULLOP() {
  return getToken(CypherParser::NULLOP, 0);
}

CypherParser::OC_MapLiteralContext* CypherParser::OC_LiteralContext::oC_MapLiteral() {
  return getRuleContext<CypherParser::OC_MapLiteralContext>(0);
}

CypherParser::OC_ListLiteralContext* CypherParser::OC_LiteralContext::oC_ListLiteral() {
  return getRuleContext<CypherParser::OC_ListLiteralContext>(0);
}


size_t CypherParser::OC_LiteralContext::getRuleIndex() const {
  return CypherParser::RuleOC_Literal;
}

void CypherParser::OC_LiteralContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_Literal(this);
}

void CypherParser::OC_LiteralContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_Literal(this);
}

CypherParser::OC_LiteralContext* CypherParser::oC_Literal() {
  OC_LiteralContext *_localctx = _tracker.createInstance<OC_LiteralContext>(_ctx, getState());
  enterRule(_localctx, 130, CypherParser::RuleOC_Literal);

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    setState(1171);
    _errHandler->sync(this);
    switch (_input->LA(1)) {
      case CypherParser::HexInteger:
      case CypherParser::DecimalInteger:
      case CypherParser::OctalInteger:
      case CypherParser::ExponentDecimalReal:
      case CypherParser::RegularDecimalReal: {
        enterOuterAlt(_localctx, 1);
        setState(1165);
        oC_NumberLiteral();
        break;
      }

      case CypherParser::StringLiteral: {
        enterOuterAlt(_localctx, 2);
        setState(1166);
        match(CypherParser::StringLiteral);
        break;
      }

      case CypherParser::TRUE:
      case CypherParser::FALSE: {
        enterOuterAlt(_localctx, 3);
        setState(1167);
        oC_BooleanLiteral();
        break;
      }

      case CypherParser::NULLOP: {
        enterOuterAlt(_localctx, 4);
        setState(1168);
        match(CypherParser::NULLOP);
        break;
      }

      case CypherParser::T__23: {
        enterOuterAlt(_localctx, 5);
        setState(1169);
        oC_MapLiteral();
        break;
      }

      case CypherParser::T__7: {
        enterOuterAlt(_localctx, 6);
        setState(1170);
        oC_ListLiteral();
        break;
      }

    default:
      throw NoViableAltException(this);
    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_BooleanLiteralContext ------------------------------------------------------------------

CypherParser::OC_BooleanLiteralContext::OC_BooleanLiteralContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

tree::TerminalNode* CypherParser::OC_BooleanLiteralContext::TRUE() {
  return getToken(CypherParser::TRUE, 0);
}

tree::TerminalNode* CypherParser::OC_BooleanLiteralContext::FALSE() {
  return getToken(CypherParser::FALSE, 0);
}


size_t CypherParser::OC_BooleanLiteralContext::getRuleIndex() const {
  return CypherParser::RuleOC_BooleanLiteral;
}

void CypherParser::OC_BooleanLiteralContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_BooleanLiteral(this);
}

void CypherParser::OC_BooleanLiteralContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_BooleanLiteral(this);
}

CypherParser::OC_BooleanLiteralContext* CypherParser::oC_BooleanLiteral() {
  OC_BooleanLiteralContext *_localctx = _tracker.createInstance<OC_BooleanLiteralContext>(_ctx, getState());
  enterRule(_localctx, 132, CypherParser::RuleOC_BooleanLiteral);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(1173);
    _la = _input->LA(1);
    if (!(_la == CypherParser::TRUE

    || _la == CypherParser::FALSE)) {
    _errHandler->recoverInline(this);
    }
    else {
      _errHandler->reportMatch(this);
      consume();
    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_ListLiteralContext ------------------------------------------------------------------

CypherParser::OC_ListLiteralContext::OC_ListLiteralContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

std::vector<tree::TerminalNode *> CypherParser::OC_ListLiteralContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_ListLiteralContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}

std::vector<CypherParser::OC_ExpressionContext *> CypherParser::OC_ListLiteralContext::oC_Expression() {
  return getRuleContexts<CypherParser::OC_ExpressionContext>();
}

CypherParser::OC_ExpressionContext* CypherParser::OC_ListLiteralContext::oC_Expression(size_t i) {
  return getRuleContext<CypherParser::OC_ExpressionContext>(i);
}


size_t CypherParser::OC_ListLiteralContext::getRuleIndex() const {
  return CypherParser::RuleOC_ListLiteral;
}

void CypherParser::OC_ListLiteralContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_ListLiteral(this);
}

void CypherParser::OC_ListLiteralContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_ListLiteral(this);
}

CypherParser::OC_ListLiteralContext* CypherParser::oC_ListLiteral() {
  OC_ListLiteralContext *_localctx = _tracker.createInstance<OC_ListLiteralContext>(_ctx, getState());
  enterRule(_localctx, 134, CypherParser::RuleOC_ListLiteral);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(1175);
    match(CypherParser::T__7);
    setState(1177);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (_la == CypherParser::SP) {
      setState(1176);
      match(CypherParser::SP);
    }
    setState(1196);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if ((((_la & ~ 0x3fULL) == 0) &&
      ((1ULL << _la) & ((1ULL << CypherParser::T__5)
      | (1ULL << CypherParser::T__7)
      | (1ULL << CypherParser::T__12)
      | (1ULL << CypherParser::T__13)
      | (1ULL << CypherParser::T__23)
      | (1ULL << CypherParser::T__25)
      | (1ULL << CypherParser::ALL))) != 0) || ((((_la - 76) & ~ 0x3fULL) == 0) &&
      ((1ULL << (_la - 76)) & ((1ULL << (CypherParser::NOT - 76))
      | (1ULL << (CypherParser::NULLOP - 76))
      | (1ULL << (CypherParser::COUNT - 76))
      | (1ULL << (CypherParser::ANY - 76))
      | (1ULL << (CypherParser::NONE - 76))
      | (1ULL << (CypherParser::SINGLE - 76))
      | (1ULL << (CypherParser::TRUE - 76))
      | (1ULL << (CypherParser::FALSE - 76))
      | (1ULL << (CypherParser::EXISTS - 76))
      | (1ULL << (CypherParser::CASE - 76))
      | (1ULL << (CypherParser::StringLiteral - 76))
      | (1ULL << (CypherParser::HexInteger - 76))
      | (1ULL << (CypherParser::DecimalInteger - 76))
      | (1ULL << (CypherParser::OctalInteger - 76))
      | (1ULL << (CypherParser::HexLetter - 76))
      | (1ULL << (CypherParser::ExponentDecimalReal - 76))
      | (1ULL << (CypherParser::RegularDecimalReal - 76))
      | (1ULL << (CypherParser::FILTER - 76))
      | (1ULL << (CypherParser::EXTRACT - 76))
      | (1ULL << (CypherParser::UnescapedSymbolicName - 76))
      | (1ULL << (CypherParser::EscapedSymbolicName - 76)))) != 0)) {
      setState(1179);
      oC_Expression();
      setState(1181);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(1180);
        match(CypherParser::SP);
      }
      setState(1193);
      _errHandler->sync(this);
      _la = _input->LA(1);
      while (_la == CypherParser::T__1) {
        setState(1183);
        match(CypherParser::T__1);
        setState(1185);
        _errHandler->sync(this);

        _la = _input->LA(1);
        if (_la == CypherParser::SP) {
          setState(1184);
          match(CypherParser::SP);
        }
        setState(1187);
        oC_Expression();
        setState(1189);
        _errHandler->sync(this);

        _la = _input->LA(1);
        if (_la == CypherParser::SP) {
          setState(1188);
          match(CypherParser::SP);
        }
        setState(1195);
        _errHandler->sync(this);
        _la = _input->LA(1);
      }
    }
    setState(1198);
    match(CypherParser::T__8);
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_PartialComparisonExpressionContext ------------------------------------------------------------------

CypherParser::OC_PartialComparisonExpressionContext::OC_PartialComparisonExpressionContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

CypherParser::OC_AddOrSubtractExpressionContext* CypherParser::OC_PartialComparisonExpressionContext::oC_AddOrSubtractExpression() {
  return getRuleContext<CypherParser::OC_AddOrSubtractExpressionContext>(0);
}

tree::TerminalNode* CypherParser::OC_PartialComparisonExpressionContext::SP() {
  return getToken(CypherParser::SP, 0);
}


size_t CypherParser::OC_PartialComparisonExpressionContext::getRuleIndex() const {
  return CypherParser::RuleOC_PartialComparisonExpression;
}

void CypherParser::OC_PartialComparisonExpressionContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_PartialComparisonExpression(this);
}

void CypherParser::OC_PartialComparisonExpressionContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_PartialComparisonExpression(this);
}

CypherParser::OC_PartialComparisonExpressionContext* CypherParser::oC_PartialComparisonExpression() {
  OC_PartialComparisonExpressionContext *_localctx = _tracker.createInstance<OC_PartialComparisonExpressionContext>(_ctx, getState());
  enterRule(_localctx, 136, CypherParser::RuleOC_PartialComparisonExpression);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    setState(1230);
    _errHandler->sync(this);
    switch (_input->LA(1)) {
      case CypherParser::T__2: {
        enterOuterAlt(_localctx, 1);
        setState(1200);
        match(CypherParser::T__2);
        setState(1202);
        _errHandler->sync(this);

        _la = _input->LA(1);
        if (_la == CypherParser::SP) {
          setState(1201);
          match(CypherParser::SP);
        }
        setState(1204);
        oC_AddOrSubtractExpression();
        break;
      }

      case CypherParser::T__17: {
        enterOuterAlt(_localctx, 2);
        setState(1205);
        match(CypherParser::T__17);
        setState(1207);
        _errHandler->sync(this);

        _la = _input->LA(1);
        if (_la == CypherParser::SP) {
          setState(1206);
          match(CypherParser::SP);
        }
        setState(1209);
        oC_AddOrSubtractExpression();
        break;
      }

      case CypherParser::T__18: {
        enterOuterAlt(_localctx, 3);
        setState(1210);
        match(CypherParser::T__18);
        setState(1212);
        _errHandler->sync(this);

        _la = _input->LA(1);
        if (_la == CypherParser::SP) {
          setState(1211);
          match(CypherParser::SP);
        }
        setState(1214);
        oC_AddOrSubtractExpression();
        break;
      }

      case CypherParser::T__19: {
        enterOuterAlt(_localctx, 4);
        setState(1215);
        match(CypherParser::T__19);
        setState(1217);
        _errHandler->sync(this);

        _la = _input->LA(1);
        if (_la == CypherParser::SP) {
          setState(1216);
          match(CypherParser::SP);
        }
        setState(1219);
        oC_AddOrSubtractExpression();
        break;
      }

      case CypherParser::T__20: {
        enterOuterAlt(_localctx, 5);
        setState(1220);
        match(CypherParser::T__20);
        setState(1222);
        _errHandler->sync(this);

        _la = _input->LA(1);
        if (_la == CypherParser::SP) {
          setState(1221);
          match(CypherParser::SP);
        }
        setState(1224);
        oC_AddOrSubtractExpression();
        break;
      }

      case CypherParser::T__21: {
        enterOuterAlt(_localctx, 6);
        setState(1225);
        match(CypherParser::T__21);
        setState(1227);
        _errHandler->sync(this);

        _la = _input->LA(1);
        if (_la == CypherParser::SP) {
          setState(1226);
          match(CypherParser::SP);
        }
        setState(1229);
        oC_AddOrSubtractExpression();
        break;
      }

    default:
      throw NoViableAltException(this);
    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_ParenthesizedExpressionContext ------------------------------------------------------------------

CypherParser::OC_ParenthesizedExpressionContext::OC_ParenthesizedExpressionContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

CypherParser::OC_ExpressionContext* CypherParser::OC_ParenthesizedExpressionContext::oC_Expression() {
  return getRuleContext<CypherParser::OC_ExpressionContext>(0);
}

std::vector<tree::TerminalNode *> CypherParser::OC_ParenthesizedExpressionContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_ParenthesizedExpressionContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}


size_t CypherParser::OC_ParenthesizedExpressionContext::getRuleIndex() const {
  return CypherParser::RuleOC_ParenthesizedExpression;
}

void CypherParser::OC_ParenthesizedExpressionContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_ParenthesizedExpression(this);
}

void CypherParser::OC_ParenthesizedExpressionContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_ParenthesizedExpression(this);
}

CypherParser::OC_ParenthesizedExpressionContext* CypherParser::oC_ParenthesizedExpression() {
  OC_ParenthesizedExpressionContext *_localctx = _tracker.createInstance<OC_ParenthesizedExpressionContext>(_ctx, getState());
  enterRule(_localctx, 138, CypherParser::RuleOC_ParenthesizedExpression);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(1232);
    match(CypherParser::T__5);
    setState(1234);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (_la == CypherParser::SP) {
      setState(1233);
      match(CypherParser::SP);
    }
    setState(1236);
    oC_Expression();
    setState(1238);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (_la == CypherParser::SP) {
      setState(1237);
      match(CypherParser::SP);
    }
    setState(1240);
    match(CypherParser::T__6);
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_RelationshipsPatternContext ------------------------------------------------------------------

CypherParser::OC_RelationshipsPatternContext::OC_RelationshipsPatternContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

CypherParser::OC_NodePatternContext* CypherParser::OC_RelationshipsPatternContext::oC_NodePattern() {
  return getRuleContext<CypherParser::OC_NodePatternContext>(0);
}

std::vector<CypherParser::OC_PatternElementChainContext *> CypherParser::OC_RelationshipsPatternContext::oC_PatternElementChain() {
  return getRuleContexts<CypherParser::OC_PatternElementChainContext>();
}

CypherParser::OC_PatternElementChainContext* CypherParser::OC_RelationshipsPatternContext::oC_PatternElementChain(size_t i) {
  return getRuleContext<CypherParser::OC_PatternElementChainContext>(i);
}

std::vector<tree::TerminalNode *> CypherParser::OC_RelationshipsPatternContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_RelationshipsPatternContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}


size_t CypherParser::OC_RelationshipsPatternContext::getRuleIndex() const {
  return CypherParser::RuleOC_RelationshipsPattern;
}

void CypherParser::OC_RelationshipsPatternContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_RelationshipsPattern(this);
}

void CypherParser::OC_RelationshipsPatternContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_RelationshipsPattern(this);
}

CypherParser::OC_RelationshipsPatternContext* CypherParser::oC_RelationshipsPattern() {
  OC_RelationshipsPatternContext *_localctx = _tracker.createInstance<OC_RelationshipsPatternContext>(_ctx, getState());
  enterRule(_localctx, 140, CypherParser::RuleOC_RelationshipsPattern);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    size_t alt;
    enterOuterAlt(_localctx, 1);
    setState(1242);
    oC_NodePattern();
    setState(1247); 
    _errHandler->sync(this);
    alt = 1;
    do {
      switch (alt) {
        case 1: {
              setState(1244);
              _errHandler->sync(this);

              _la = _input->LA(1);
              if (_la == CypherParser::SP) {
                setState(1243);
                match(CypherParser::SP);
              }
              setState(1246);
              oC_PatternElementChain();
              break;
            }

      default:
        throw NoViableAltException(this);
      }
      setState(1249); 
      _errHandler->sync(this);
      alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 220, _ctx);
    } while (alt != 2 && alt != atn::ATN::INVALID_ALT_NUMBER);
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_FilterExpressionContext ------------------------------------------------------------------

CypherParser::OC_FilterExpressionContext::OC_FilterExpressionContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

CypherParser::OC_IdInCollContext* CypherParser::OC_FilterExpressionContext::oC_IdInColl() {
  return getRuleContext<CypherParser::OC_IdInCollContext>(0);
}

CypherParser::OC_WhereContext* CypherParser::OC_FilterExpressionContext::oC_Where() {
  return getRuleContext<CypherParser::OC_WhereContext>(0);
}

tree::TerminalNode* CypherParser::OC_FilterExpressionContext::SP() {
  return getToken(CypherParser::SP, 0);
}


size_t CypherParser::OC_FilterExpressionContext::getRuleIndex() const {
  return CypherParser::RuleOC_FilterExpression;
}

void CypherParser::OC_FilterExpressionContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_FilterExpression(this);
}

void CypherParser::OC_FilterExpressionContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_FilterExpression(this);
}

CypherParser::OC_FilterExpressionContext* CypherParser::oC_FilterExpression() {
  OC_FilterExpressionContext *_localctx = _tracker.createInstance<OC_FilterExpressionContext>(_ctx, getState());
  enterRule(_localctx, 142, CypherParser::RuleOC_FilterExpression);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(1251);
    oC_IdInColl();
    setState(1256);
    _errHandler->sync(this);

    switch (getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 222, _ctx)) {
    case 1: {
      setState(1253);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(1252);
        match(CypherParser::SP);
      }
      setState(1255);
      oC_Where();
      break;
    }

    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_IdInCollContext ------------------------------------------------------------------

CypherParser::OC_IdInCollContext::OC_IdInCollContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

CypherParser::OC_VariableContext* CypherParser::OC_IdInCollContext::oC_Variable() {
  return getRuleContext<CypherParser::OC_VariableContext>(0);
}

std::vector<tree::TerminalNode *> CypherParser::OC_IdInCollContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_IdInCollContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}

tree::TerminalNode* CypherParser::OC_IdInCollContext::IN() {
  return getToken(CypherParser::IN, 0);
}

CypherParser::OC_ExpressionContext* CypherParser::OC_IdInCollContext::oC_Expression() {
  return getRuleContext<CypherParser::OC_ExpressionContext>(0);
}


size_t CypherParser::OC_IdInCollContext::getRuleIndex() const {
  return CypherParser::RuleOC_IdInColl;
}

void CypherParser::OC_IdInCollContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_IdInColl(this);
}

void CypherParser::OC_IdInCollContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_IdInColl(this);
}

CypherParser::OC_IdInCollContext* CypherParser::oC_IdInColl() {
  OC_IdInCollContext *_localctx = _tracker.createInstance<OC_IdInCollContext>(_ctx, getState());
  enterRule(_localctx, 144, CypherParser::RuleOC_IdInColl);

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(1258);
    oC_Variable();
    setState(1259);
    match(CypherParser::SP);
    setState(1260);
    match(CypherParser::IN);
    setState(1261);
    match(CypherParser::SP);
    setState(1262);
    oC_Expression();
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_FunctionInvocationContext ------------------------------------------------------------------

CypherParser::OC_FunctionInvocationContext::OC_FunctionInvocationContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

CypherParser::OC_FunctionNameContext* CypherParser::OC_FunctionInvocationContext::oC_FunctionName() {
  return getRuleContext<CypherParser::OC_FunctionNameContext>(0);
}

std::vector<tree::TerminalNode *> CypherParser::OC_FunctionInvocationContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_FunctionInvocationContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}

tree::TerminalNode* CypherParser::OC_FunctionInvocationContext::DISTINCT() {
  return getToken(CypherParser::DISTINCT, 0);
}

std::vector<CypherParser::OC_ExpressionContext *> CypherParser::OC_FunctionInvocationContext::oC_Expression() {
  return getRuleContexts<CypherParser::OC_ExpressionContext>();
}

CypherParser::OC_ExpressionContext* CypherParser::OC_FunctionInvocationContext::oC_Expression(size_t i) {
  return getRuleContext<CypherParser::OC_ExpressionContext>(i);
}


size_t CypherParser::OC_FunctionInvocationContext::getRuleIndex() const {
  return CypherParser::RuleOC_FunctionInvocation;
}

void CypherParser::OC_FunctionInvocationContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_FunctionInvocation(this);
}

void CypherParser::OC_FunctionInvocationContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_FunctionInvocation(this);
}

CypherParser::OC_FunctionInvocationContext* CypherParser::oC_FunctionInvocation() {
  OC_FunctionInvocationContext *_localctx = _tracker.createInstance<OC_FunctionInvocationContext>(_ctx, getState());
  enterRule(_localctx, 146, CypherParser::RuleOC_FunctionInvocation);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(1264);
    oC_FunctionName();
    setState(1266);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (_la == CypherParser::SP) {
      setState(1265);
      match(CypherParser::SP);
    }
    setState(1268);
    match(CypherParser::T__5);
    setState(1270);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (_la == CypherParser::SP) {
      setState(1269);
      match(CypherParser::SP);
    }
    setState(1276);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (_la == CypherParser::DISTINCT) {
      setState(1272);
      match(CypherParser::DISTINCT);
      setState(1274);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(1273);
        match(CypherParser::SP);
      }
    }
    setState(1295);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if ((((_la & ~ 0x3fULL) == 0) &&
      ((1ULL << _la) & ((1ULL << CypherParser::T__5)
      | (1ULL << CypherParser::T__7)
      | (1ULL << CypherParser::T__12)
      | (1ULL << CypherParser::T__13)
      | (1ULL << CypherParser::T__23)
      | (1ULL << CypherParser::T__25)
      | (1ULL << CypherParser::ALL))) != 0) || ((((_la - 76) & ~ 0x3fULL) == 0) &&
      ((1ULL << (_la - 76)) & ((1ULL << (CypherParser::NOT - 76))
      | (1ULL << (CypherParser::NULLOP - 76))
      | (1ULL << (CypherParser::COUNT - 76))
      | (1ULL << (CypherParser::ANY - 76))
      | (1ULL << (CypherParser::NONE - 76))
      | (1ULL << (CypherParser::SINGLE - 76))
      | (1ULL << (CypherParser::TRUE - 76))
      | (1ULL << (CypherParser::FALSE - 76))
      | (1ULL << (CypherParser::EXISTS - 76))
      | (1ULL << (CypherParser::CASE - 76))
      | (1ULL << (CypherParser::StringLiteral - 76))
      | (1ULL << (CypherParser::HexInteger - 76))
      | (1ULL << (CypherParser::DecimalInteger - 76))
      | (1ULL << (CypherParser::OctalInteger - 76))
      | (1ULL << (CypherParser::HexLetter - 76))
      | (1ULL << (CypherParser::ExponentDecimalReal - 76))
      | (1ULL << (CypherParser::RegularDecimalReal - 76))
      | (1ULL << (CypherParser::FILTER - 76))
      | (1ULL << (CypherParser::EXTRACT - 76))
      | (1ULL << (CypherParser::UnescapedSymbolicName - 76))
      | (1ULL << (CypherParser::EscapedSymbolicName - 76)))) != 0)) {
      setState(1278);
      oC_Expression();
      setState(1280);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(1279);
        match(CypherParser::SP);
      }
      setState(1292);
      _errHandler->sync(this);
      _la = _input->LA(1);
      while (_la == CypherParser::T__1) {
        setState(1282);
        match(CypherParser::T__1);
        setState(1284);
        _errHandler->sync(this);

        _la = _input->LA(1);
        if (_la == CypherParser::SP) {
          setState(1283);
          match(CypherParser::SP);
        }
        setState(1286);
        oC_Expression();
        setState(1288);
        _errHandler->sync(this);

        _la = _input->LA(1);
        if (_la == CypherParser::SP) {
          setState(1287);
          match(CypherParser::SP);
        }
        setState(1294);
        _errHandler->sync(this);
        _la = _input->LA(1);
      }
    }
    setState(1297);
    match(CypherParser::T__6);
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_FunctionNameContext ------------------------------------------------------------------

CypherParser::OC_FunctionNameContext::OC_FunctionNameContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

CypherParser::OC_NamespaceContext* CypherParser::OC_FunctionNameContext::oC_Namespace() {
  return getRuleContext<CypherParser::OC_NamespaceContext>(0);
}

CypherParser::OC_SymbolicNameContext* CypherParser::OC_FunctionNameContext::oC_SymbolicName() {
  return getRuleContext<CypherParser::OC_SymbolicNameContext>(0);
}

tree::TerminalNode* CypherParser::OC_FunctionNameContext::EXISTS() {
  return getToken(CypherParser::EXISTS, 0);
}


size_t CypherParser::OC_FunctionNameContext::getRuleIndex() const {
  return CypherParser::RuleOC_FunctionName;
}

void CypherParser::OC_FunctionNameContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_FunctionName(this);
}

void CypherParser::OC_FunctionNameContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_FunctionName(this);
}

CypherParser::OC_FunctionNameContext* CypherParser::oC_FunctionName() {
  OC_FunctionNameContext *_localctx = _tracker.createInstance<OC_FunctionNameContext>(_ctx, getState());
  enterRule(_localctx, 148, CypherParser::RuleOC_FunctionName);

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    setState(1303);
    _errHandler->sync(this);
    switch (_input->LA(1)) {
      case CypherParser::COUNT:
      case CypherParser::ANY:
      case CypherParser::NONE:
      case CypherParser::SINGLE:
      case CypherParser::HexLetter:
      case CypherParser::FILTER:
      case CypherParser::EXTRACT:
      case CypherParser::UnescapedSymbolicName:
      case CypherParser::EscapedSymbolicName: {
        enterOuterAlt(_localctx, 1);
        setState(1299);
        oC_Namespace();
        setState(1300);
        oC_SymbolicName();
        break;
      }

      case CypherParser::EXISTS: {
        enterOuterAlt(_localctx, 2);
        setState(1302);
        match(CypherParser::EXISTS);
        break;
      }

    default:
      throw NoViableAltException(this);
    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_ExplicitProcedureInvocationContext ------------------------------------------------------------------

CypherParser::OC_ExplicitProcedureInvocationContext::OC_ExplicitProcedureInvocationContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

CypherParser::OC_ProcedureNameContext* CypherParser::OC_ExplicitProcedureInvocationContext::oC_ProcedureName() {
  return getRuleContext<CypherParser::OC_ProcedureNameContext>(0);
}

std::vector<tree::TerminalNode *> CypherParser::OC_ExplicitProcedureInvocationContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_ExplicitProcedureInvocationContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}

std::vector<CypherParser::OC_ExpressionContext *> CypherParser::OC_ExplicitProcedureInvocationContext::oC_Expression() {
  return getRuleContexts<CypherParser::OC_ExpressionContext>();
}

CypherParser::OC_ExpressionContext* CypherParser::OC_ExplicitProcedureInvocationContext::oC_Expression(size_t i) {
  return getRuleContext<CypherParser::OC_ExpressionContext>(i);
}


size_t CypherParser::OC_ExplicitProcedureInvocationContext::getRuleIndex() const {
  return CypherParser::RuleOC_ExplicitProcedureInvocation;
}

void CypherParser::OC_ExplicitProcedureInvocationContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_ExplicitProcedureInvocation(this);
}

void CypherParser::OC_ExplicitProcedureInvocationContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_ExplicitProcedureInvocation(this);
}

CypherParser::OC_ExplicitProcedureInvocationContext* CypherParser::oC_ExplicitProcedureInvocation() {
  OC_ExplicitProcedureInvocationContext *_localctx = _tracker.createInstance<OC_ExplicitProcedureInvocationContext>(_ctx, getState());
  enterRule(_localctx, 150, CypherParser::RuleOC_ExplicitProcedureInvocation);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(1305);
    oC_ProcedureName();
    setState(1307);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (_la == CypherParser::SP) {
      setState(1306);
      match(CypherParser::SP);
    }
    setState(1309);
    match(CypherParser::T__5);
    setState(1311);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (_la == CypherParser::SP) {
      setState(1310);
      match(CypherParser::SP);
    }
    setState(1330);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if ((((_la & ~ 0x3fULL) == 0) &&
      ((1ULL << _la) & ((1ULL << CypherParser::T__5)
      | (1ULL << CypherParser::T__7)
      | (1ULL << CypherParser::T__12)
      | (1ULL << CypherParser::T__13)
      | (1ULL << CypherParser::T__23)
      | (1ULL << CypherParser::T__25)
      | (1ULL << CypherParser::ALL))) != 0) || ((((_la - 76) & ~ 0x3fULL) == 0) &&
      ((1ULL << (_la - 76)) & ((1ULL << (CypherParser::NOT - 76))
      | (1ULL << (CypherParser::NULLOP - 76))
      | (1ULL << (CypherParser::COUNT - 76))
      | (1ULL << (CypherParser::ANY - 76))
      | (1ULL << (CypherParser::NONE - 76))
      | (1ULL << (CypherParser::SINGLE - 76))
      | (1ULL << (CypherParser::TRUE - 76))
      | (1ULL << (CypherParser::FALSE - 76))
      | (1ULL << (CypherParser::EXISTS - 76))
      | (1ULL << (CypherParser::CASE - 76))
      | (1ULL << (CypherParser::StringLiteral - 76))
      | (1ULL << (CypherParser::HexInteger - 76))
      | (1ULL << (CypherParser::DecimalInteger - 76))
      | (1ULL << (CypherParser::OctalInteger - 76))
      | (1ULL << (CypherParser::HexLetter - 76))
      | (1ULL << (CypherParser::ExponentDecimalReal - 76))
      | (1ULL << (CypherParser::RegularDecimalReal - 76))
      | (1ULL << (CypherParser::FILTER - 76))
      | (1ULL << (CypherParser::EXTRACT - 76))
      | (1ULL << (CypherParser::UnescapedSymbolicName - 76))
      | (1ULL << (CypherParser::EscapedSymbolicName - 76)))) != 0)) {
      setState(1313);
      oC_Expression();
      setState(1315);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(1314);
        match(CypherParser::SP);
      }
      setState(1327);
      _errHandler->sync(this);
      _la = _input->LA(1);
      while (_la == CypherParser::T__1) {
        setState(1317);
        match(CypherParser::T__1);
        setState(1319);
        _errHandler->sync(this);

        _la = _input->LA(1);
        if (_la == CypherParser::SP) {
          setState(1318);
          match(CypherParser::SP);
        }
        setState(1321);
        oC_Expression();
        setState(1323);
        _errHandler->sync(this);

        _la = _input->LA(1);
        if (_la == CypherParser::SP) {
          setState(1322);
          match(CypherParser::SP);
        }
        setState(1329);
        _errHandler->sync(this);
        _la = _input->LA(1);
      }
    }
    setState(1332);
    match(CypherParser::T__6);
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_ImplicitProcedureInvocationContext ------------------------------------------------------------------

CypherParser::OC_ImplicitProcedureInvocationContext::OC_ImplicitProcedureInvocationContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

CypherParser::OC_ProcedureNameContext* CypherParser::OC_ImplicitProcedureInvocationContext::oC_ProcedureName() {
  return getRuleContext<CypherParser::OC_ProcedureNameContext>(0);
}


size_t CypherParser::OC_ImplicitProcedureInvocationContext::getRuleIndex() const {
  return CypherParser::RuleOC_ImplicitProcedureInvocation;
}

void CypherParser::OC_ImplicitProcedureInvocationContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_ImplicitProcedureInvocation(this);
}

void CypherParser::OC_ImplicitProcedureInvocationContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_ImplicitProcedureInvocation(this);
}

CypherParser::OC_ImplicitProcedureInvocationContext* CypherParser::oC_ImplicitProcedureInvocation() {
  OC_ImplicitProcedureInvocationContext *_localctx = _tracker.createInstance<OC_ImplicitProcedureInvocationContext>(_ctx, getState());
  enterRule(_localctx, 152, CypherParser::RuleOC_ImplicitProcedureInvocation);

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(1334);
    oC_ProcedureName();
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_ProcedureResultFieldContext ------------------------------------------------------------------

CypherParser::OC_ProcedureResultFieldContext::OC_ProcedureResultFieldContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

CypherParser::OC_SymbolicNameContext* CypherParser::OC_ProcedureResultFieldContext::oC_SymbolicName() {
  return getRuleContext<CypherParser::OC_SymbolicNameContext>(0);
}


size_t CypherParser::OC_ProcedureResultFieldContext::getRuleIndex() const {
  return CypherParser::RuleOC_ProcedureResultField;
}

void CypherParser::OC_ProcedureResultFieldContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_ProcedureResultField(this);
}

void CypherParser::OC_ProcedureResultFieldContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_ProcedureResultField(this);
}

CypherParser::OC_ProcedureResultFieldContext* CypherParser::oC_ProcedureResultField() {
  OC_ProcedureResultFieldContext *_localctx = _tracker.createInstance<OC_ProcedureResultFieldContext>(_ctx, getState());
  enterRule(_localctx, 154, CypherParser::RuleOC_ProcedureResultField);

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(1336);
    oC_SymbolicName();
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_ProcedureNameContext ------------------------------------------------------------------

CypherParser::OC_ProcedureNameContext::OC_ProcedureNameContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

CypherParser::OC_NamespaceContext* CypherParser::OC_ProcedureNameContext::oC_Namespace() {
  return getRuleContext<CypherParser::OC_NamespaceContext>(0);
}

CypherParser::OC_SymbolicNameContext* CypherParser::OC_ProcedureNameContext::oC_SymbolicName() {
  return getRuleContext<CypherParser::OC_SymbolicNameContext>(0);
}


size_t CypherParser::OC_ProcedureNameContext::getRuleIndex() const {
  return CypherParser::RuleOC_ProcedureName;
}

void CypherParser::OC_ProcedureNameContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_ProcedureName(this);
}

void CypherParser::OC_ProcedureNameContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_ProcedureName(this);
}

CypherParser::OC_ProcedureNameContext* CypherParser::oC_ProcedureName() {
  OC_ProcedureNameContext *_localctx = _tracker.createInstance<OC_ProcedureNameContext>(_ctx, getState());
  enterRule(_localctx, 156, CypherParser::RuleOC_ProcedureName);

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(1338);
    oC_Namespace();
    setState(1339);
    oC_SymbolicName();
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_NamespaceContext ------------------------------------------------------------------

CypherParser::OC_NamespaceContext::OC_NamespaceContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

std::vector<CypherParser::OC_SymbolicNameContext *> CypherParser::OC_NamespaceContext::oC_SymbolicName() {
  return getRuleContexts<CypherParser::OC_SymbolicNameContext>();
}

CypherParser::OC_SymbolicNameContext* CypherParser::OC_NamespaceContext::oC_SymbolicName(size_t i) {
  return getRuleContext<CypherParser::OC_SymbolicNameContext>(i);
}


size_t CypherParser::OC_NamespaceContext::getRuleIndex() const {
  return CypherParser::RuleOC_Namespace;
}

void CypherParser::OC_NamespaceContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_Namespace(this);
}

void CypherParser::OC_NamespaceContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_Namespace(this);
}

CypherParser::OC_NamespaceContext* CypherParser::oC_Namespace() {
  OC_NamespaceContext *_localctx = _tracker.createInstance<OC_NamespaceContext>(_ctx, getState());
  enterRule(_localctx, 158, CypherParser::RuleOC_Namespace);

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    size_t alt;
    enterOuterAlt(_localctx, 1);
    setState(1346);
    _errHandler->sync(this);
    alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 240, _ctx);
    while (alt != 2 && alt != atn::ATN::INVALID_ALT_NUMBER) {
      if (alt == 1) {
        setState(1341);
        oC_SymbolicName();
        setState(1342);
        match(CypherParser::T__22); 
      }
      setState(1348);
      _errHandler->sync(this);
      alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 240, _ctx);
    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_ListComprehensionContext ------------------------------------------------------------------

CypherParser::OC_ListComprehensionContext::OC_ListComprehensionContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

CypherParser::OC_FilterExpressionContext* CypherParser::OC_ListComprehensionContext::oC_FilterExpression() {
  return getRuleContext<CypherParser::OC_FilterExpressionContext>(0);
}

std::vector<tree::TerminalNode *> CypherParser::OC_ListComprehensionContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_ListComprehensionContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}

CypherParser::OC_ExpressionContext* CypherParser::OC_ListComprehensionContext::oC_Expression() {
  return getRuleContext<CypherParser::OC_ExpressionContext>(0);
}


size_t CypherParser::OC_ListComprehensionContext::getRuleIndex() const {
  return CypherParser::RuleOC_ListComprehension;
}

void CypherParser::OC_ListComprehensionContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_ListComprehension(this);
}

void CypherParser::OC_ListComprehensionContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_ListComprehension(this);
}

CypherParser::OC_ListComprehensionContext* CypherParser::oC_ListComprehension() {
  OC_ListComprehensionContext *_localctx = _tracker.createInstance<OC_ListComprehensionContext>(_ctx, getState());
  enterRule(_localctx, 160, CypherParser::RuleOC_ListComprehension);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(1349);
    match(CypherParser::T__7);
    setState(1351);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (_la == CypherParser::SP) {
      setState(1350);
      match(CypherParser::SP);
    }
    setState(1353);
    oC_FilterExpression();
    setState(1362);
    _errHandler->sync(this);

    switch (getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 244, _ctx)) {
    case 1: {
      setState(1355);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(1354);
        match(CypherParser::SP);
      }
      setState(1357);
      match(CypherParser::T__10);
      setState(1359);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(1358);
        match(CypherParser::SP);
      }
      setState(1361);
      oC_Expression();
      break;
    }

    }
    setState(1365);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (_la == CypherParser::SP) {
      setState(1364);
      match(CypherParser::SP);
    }
    setState(1367);
    match(CypherParser::T__8);
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_PatternComprehensionContext ------------------------------------------------------------------

CypherParser::OC_PatternComprehensionContext::OC_PatternComprehensionContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

CypherParser::OC_RelationshipsPatternContext* CypherParser::OC_PatternComprehensionContext::oC_RelationshipsPattern() {
  return getRuleContext<CypherParser::OC_RelationshipsPatternContext>(0);
}

std::vector<CypherParser::OC_ExpressionContext *> CypherParser::OC_PatternComprehensionContext::oC_Expression() {
  return getRuleContexts<CypherParser::OC_ExpressionContext>();
}

CypherParser::OC_ExpressionContext* CypherParser::OC_PatternComprehensionContext::oC_Expression(size_t i) {
  return getRuleContext<CypherParser::OC_ExpressionContext>(i);
}

std::vector<tree::TerminalNode *> CypherParser::OC_PatternComprehensionContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_PatternComprehensionContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}

CypherParser::OC_VariableContext* CypherParser::OC_PatternComprehensionContext::oC_Variable() {
  return getRuleContext<CypherParser::OC_VariableContext>(0);
}

tree::TerminalNode* CypherParser::OC_PatternComprehensionContext::WHERE() {
  return getToken(CypherParser::WHERE, 0);
}


size_t CypherParser::OC_PatternComprehensionContext::getRuleIndex() const {
  return CypherParser::RuleOC_PatternComprehension;
}

void CypherParser::OC_PatternComprehensionContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_PatternComprehension(this);
}

void CypherParser::OC_PatternComprehensionContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_PatternComprehension(this);
}

CypherParser::OC_PatternComprehensionContext* CypherParser::oC_PatternComprehension() {
  OC_PatternComprehensionContext *_localctx = _tracker.createInstance<OC_PatternComprehensionContext>(_ctx, getState());
  enterRule(_localctx, 162, CypherParser::RuleOC_PatternComprehension);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(1369);
    match(CypherParser::T__7);
    setState(1371);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (_la == CypherParser::SP) {
      setState(1370);
      match(CypherParser::SP);
    }
    setState(1381);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (((((_la - 83) & ~ 0x3fULL) == 0) &&
      ((1ULL << (_la - 83)) & ((1ULL << (CypherParser::COUNT - 83))
      | (1ULL << (CypherParser::ANY - 83))
      | (1ULL << (CypherParser::NONE - 83))
      | (1ULL << (CypherParser::SINGLE - 83))
      | (1ULL << (CypherParser::HexLetter - 83))
      | (1ULL << (CypherParser::FILTER - 83))
      | (1ULL << (CypherParser::EXTRACT - 83))
      | (1ULL << (CypherParser::UnescapedSymbolicName - 83))
      | (1ULL << (CypherParser::EscapedSymbolicName - 83)))) != 0)) {
      setState(1373);
      oC_Variable();
      setState(1375);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(1374);
        match(CypherParser::SP);
      }
      setState(1377);
      match(CypherParser::T__2);
      setState(1379);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(1378);
        match(CypherParser::SP);
      }
    }
    setState(1383);
    oC_RelationshipsPattern();
    setState(1385);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (_la == CypherParser::SP) {
      setState(1384);
      match(CypherParser::SP);
    }
    setState(1395);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (_la == CypherParser::WHERE) {
      setState(1387);
      match(CypherParser::WHERE);
      setState(1389);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(1388);
        match(CypherParser::SP);
      }
      setState(1391);
      oC_Expression();
      setState(1393);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(1392);
        match(CypherParser::SP);
      }
    }
    setState(1397);
    match(CypherParser::T__10);
    setState(1399);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (_la == CypherParser::SP) {
      setState(1398);
      match(CypherParser::SP);
    }
    setState(1401);
    oC_Expression();
    setState(1403);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (_la == CypherParser::SP) {
      setState(1402);
      match(CypherParser::SP);
    }
    setState(1405);
    match(CypherParser::T__8);
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_PropertyLookupContext ------------------------------------------------------------------

CypherParser::OC_PropertyLookupContext::OC_PropertyLookupContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

CypherParser::OC_PropertyKeyNameContext* CypherParser::OC_PropertyLookupContext::oC_PropertyKeyName() {
  return getRuleContext<CypherParser::OC_PropertyKeyNameContext>(0);
}

tree::TerminalNode* CypherParser::OC_PropertyLookupContext::SP() {
  return getToken(CypherParser::SP, 0);
}


size_t CypherParser::OC_PropertyLookupContext::getRuleIndex() const {
  return CypherParser::RuleOC_PropertyLookup;
}

void CypherParser::OC_PropertyLookupContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_PropertyLookup(this);
}

void CypherParser::OC_PropertyLookupContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_PropertyLookup(this);
}

CypherParser::OC_PropertyLookupContext* CypherParser::oC_PropertyLookup() {
  OC_PropertyLookupContext *_localctx = _tracker.createInstance<OC_PropertyLookupContext>(_ctx, getState());
  enterRule(_localctx, 164, CypherParser::RuleOC_PropertyLookup);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(1407);
    match(CypherParser::T__22);
    setState(1409);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (_la == CypherParser::SP) {
      setState(1408);
      match(CypherParser::SP);
    }

    setState(1411);
    oC_PropertyKeyName();
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_CaseExpressionContext ------------------------------------------------------------------

CypherParser::OC_CaseExpressionContext::OC_CaseExpressionContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

tree::TerminalNode* CypherParser::OC_CaseExpressionContext::END() {
  return getToken(CypherParser::END, 0);
}

tree::TerminalNode* CypherParser::OC_CaseExpressionContext::ELSE() {
  return getToken(CypherParser::ELSE, 0);
}

std::vector<CypherParser::OC_ExpressionContext *> CypherParser::OC_CaseExpressionContext::oC_Expression() {
  return getRuleContexts<CypherParser::OC_ExpressionContext>();
}

CypherParser::OC_ExpressionContext* CypherParser::OC_CaseExpressionContext::oC_Expression(size_t i) {
  return getRuleContext<CypherParser::OC_ExpressionContext>(i);
}

std::vector<tree::TerminalNode *> CypherParser::OC_CaseExpressionContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_CaseExpressionContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}

tree::TerminalNode* CypherParser::OC_CaseExpressionContext::CASE() {
  return getToken(CypherParser::CASE, 0);
}

std::vector<CypherParser::OC_CaseAlternativesContext *> CypherParser::OC_CaseExpressionContext::oC_CaseAlternatives() {
  return getRuleContexts<CypherParser::OC_CaseAlternativesContext>();
}

CypherParser::OC_CaseAlternativesContext* CypherParser::OC_CaseExpressionContext::oC_CaseAlternatives(size_t i) {
  return getRuleContext<CypherParser::OC_CaseAlternativesContext>(i);
}


size_t CypherParser::OC_CaseExpressionContext::getRuleIndex() const {
  return CypherParser::RuleOC_CaseExpression;
}

void CypherParser::OC_CaseExpressionContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_CaseExpression(this);
}

void CypherParser::OC_CaseExpressionContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_CaseExpression(this);
}

CypherParser::OC_CaseExpressionContext* CypherParser::oC_CaseExpression() {
  OC_CaseExpressionContext *_localctx = _tracker.createInstance<OC_CaseExpressionContext>(_ctx, getState());
  enterRule(_localctx, 166, CypherParser::RuleOC_CaseExpression);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    size_t alt;
    enterOuterAlt(_localctx, 1);
    setState(1435);
    _errHandler->sync(this);
    switch (getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 262, _ctx)) {
    case 1: {
      setState(1413);
      match(CypherParser::CASE);
      setState(1418); 
      _errHandler->sync(this);
      alt = 1;
      do {
        switch (alt) {
          case 1: {
                setState(1415);
                _errHandler->sync(this);

                _la = _input->LA(1);
                if (_la == CypherParser::SP) {
                  setState(1414);
                  match(CypherParser::SP);
                }
                setState(1417);
                oC_CaseAlternatives();
                break;
              }

        default:
          throw NoViableAltException(this);
        }
        setState(1420); 
        _errHandler->sync(this);
        alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 258, _ctx);
      } while (alt != 2 && alt != atn::ATN::INVALID_ALT_NUMBER);
      break;
    }

    case 2: {
      setState(1422);
      match(CypherParser::CASE);
      setState(1424);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(1423);
        match(CypherParser::SP);
      }
      setState(1426);
      oC_Expression();
      setState(1431); 
      _errHandler->sync(this);
      alt = 1;
      do {
        switch (alt) {
          case 1: {
                setState(1428);
                _errHandler->sync(this);

                _la = _input->LA(1);
                if (_la == CypherParser::SP) {
                  setState(1427);
                  match(CypherParser::SP);
                }
                setState(1430);
                oC_CaseAlternatives();
                break;
              }

        default:
          throw NoViableAltException(this);
        }
        setState(1433); 
        _errHandler->sync(this);
        alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 261, _ctx);
      } while (alt != 2 && alt != atn::ATN::INVALID_ALT_NUMBER);
      break;
    }

    }
    setState(1445);
    _errHandler->sync(this);

    switch (getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 265, _ctx)) {
    case 1: {
      setState(1438);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(1437);
        match(CypherParser::SP);
      }
      setState(1440);
      match(CypherParser::ELSE);
      setState(1442);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(1441);
        match(CypherParser::SP);
      }
      setState(1444);
      oC_Expression();
      break;
    }

    }
    setState(1448);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (_la == CypherParser::SP) {
      setState(1447);
      match(CypherParser::SP);
    }
    setState(1450);
    match(CypherParser::END);
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_CaseAlternativesContext ------------------------------------------------------------------

CypherParser::OC_CaseAlternativesContext::OC_CaseAlternativesContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

tree::TerminalNode* CypherParser::OC_CaseAlternativesContext::WHEN() {
  return getToken(CypherParser::WHEN, 0);
}

std::vector<CypherParser::OC_ExpressionContext *> CypherParser::OC_CaseAlternativesContext::oC_Expression() {
  return getRuleContexts<CypherParser::OC_ExpressionContext>();
}

CypherParser::OC_ExpressionContext* CypherParser::OC_CaseAlternativesContext::oC_Expression(size_t i) {
  return getRuleContext<CypherParser::OC_ExpressionContext>(i);
}

tree::TerminalNode* CypherParser::OC_CaseAlternativesContext::THEN() {
  return getToken(CypherParser::THEN, 0);
}

std::vector<tree::TerminalNode *> CypherParser::OC_CaseAlternativesContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_CaseAlternativesContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}


size_t CypherParser::OC_CaseAlternativesContext::getRuleIndex() const {
  return CypherParser::RuleOC_CaseAlternatives;
}

void CypherParser::OC_CaseAlternativesContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_CaseAlternatives(this);
}

void CypherParser::OC_CaseAlternativesContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_CaseAlternatives(this);
}

CypherParser::OC_CaseAlternativesContext* CypherParser::oC_CaseAlternatives() {
  OC_CaseAlternativesContext *_localctx = _tracker.createInstance<OC_CaseAlternativesContext>(_ctx, getState());
  enterRule(_localctx, 168, CypherParser::RuleOC_CaseAlternatives);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(1452);
    match(CypherParser::WHEN);
    setState(1454);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (_la == CypherParser::SP) {
      setState(1453);
      match(CypherParser::SP);
    }
    setState(1456);
    oC_Expression();
    setState(1458);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (_la == CypherParser::SP) {
      setState(1457);
      match(CypherParser::SP);
    }
    setState(1460);
    match(CypherParser::THEN);
    setState(1462);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (_la == CypherParser::SP) {
      setState(1461);
      match(CypherParser::SP);
    }
    setState(1464);
    oC_Expression();
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_VariableContext ------------------------------------------------------------------

CypherParser::OC_VariableContext::OC_VariableContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

CypherParser::OC_SymbolicNameContext* CypherParser::OC_VariableContext::oC_SymbolicName() {
  return getRuleContext<CypherParser::OC_SymbolicNameContext>(0);
}


size_t CypherParser::OC_VariableContext::getRuleIndex() const {
  return CypherParser::RuleOC_Variable;
}

void CypherParser::OC_VariableContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_Variable(this);
}

void CypherParser::OC_VariableContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_Variable(this);
}

CypherParser::OC_VariableContext* CypherParser::oC_Variable() {
  OC_VariableContext *_localctx = _tracker.createInstance<OC_VariableContext>(_ctx, getState());
  enterRule(_localctx, 170, CypherParser::RuleOC_Variable);

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(1466);
    oC_SymbolicName();
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_NumberLiteralContext ------------------------------------------------------------------

CypherParser::OC_NumberLiteralContext::OC_NumberLiteralContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

CypherParser::OC_DoubleLiteralContext* CypherParser::OC_NumberLiteralContext::oC_DoubleLiteral() {
  return getRuleContext<CypherParser::OC_DoubleLiteralContext>(0);
}

CypherParser::OC_IntegerLiteralContext* CypherParser::OC_NumberLiteralContext::oC_IntegerLiteral() {
  return getRuleContext<CypherParser::OC_IntegerLiteralContext>(0);
}


size_t CypherParser::OC_NumberLiteralContext::getRuleIndex() const {
  return CypherParser::RuleOC_NumberLiteral;
}

void CypherParser::OC_NumberLiteralContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_NumberLiteral(this);
}

void CypherParser::OC_NumberLiteralContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_NumberLiteral(this);
}

CypherParser::OC_NumberLiteralContext* CypherParser::oC_NumberLiteral() {
  OC_NumberLiteralContext *_localctx = _tracker.createInstance<OC_NumberLiteralContext>(_ctx, getState());
  enterRule(_localctx, 172, CypherParser::RuleOC_NumberLiteral);

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    setState(1470);
    _errHandler->sync(this);
    switch (_input->LA(1)) {
      case CypherParser::ExponentDecimalReal:
      case CypherParser::RegularDecimalReal: {
        enterOuterAlt(_localctx, 1);
        setState(1468);
        oC_DoubleLiteral();
        break;
      }

      case CypherParser::HexInteger:
      case CypherParser::DecimalInteger:
      case CypherParser::OctalInteger: {
        enterOuterAlt(_localctx, 2);
        setState(1469);
        oC_IntegerLiteral();
        break;
      }

    default:
      throw NoViableAltException(this);
    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_MapLiteralContext ------------------------------------------------------------------

CypherParser::OC_MapLiteralContext::OC_MapLiteralContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

std::vector<tree::TerminalNode *> CypherParser::OC_MapLiteralContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_MapLiteralContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}

std::vector<CypherParser::OC_PropertyKeyNameContext *> CypherParser::OC_MapLiteralContext::oC_PropertyKeyName() {
  return getRuleContexts<CypherParser::OC_PropertyKeyNameContext>();
}

CypherParser::OC_PropertyKeyNameContext* CypherParser::OC_MapLiteralContext::oC_PropertyKeyName(size_t i) {
  return getRuleContext<CypherParser::OC_PropertyKeyNameContext>(i);
}

std::vector<CypherParser::OC_ExpressionContext *> CypherParser::OC_MapLiteralContext::oC_Expression() {
  return getRuleContexts<CypherParser::OC_ExpressionContext>();
}

CypherParser::OC_ExpressionContext* CypherParser::OC_MapLiteralContext::oC_Expression(size_t i) {
  return getRuleContext<CypherParser::OC_ExpressionContext>(i);
}


size_t CypherParser::OC_MapLiteralContext::getRuleIndex() const {
  return CypherParser::RuleOC_MapLiteral;
}

void CypherParser::OC_MapLiteralContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_MapLiteral(this);
}

void CypherParser::OC_MapLiteralContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_MapLiteral(this);
}

CypherParser::OC_MapLiteralContext* CypherParser::oC_MapLiteral() {
  OC_MapLiteralContext *_localctx = _tracker.createInstance<OC_MapLiteralContext>(_ctx, getState());
  enterRule(_localctx, 174, CypherParser::RuleOC_MapLiteral);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(1472);
    match(CypherParser::T__23);
    setState(1474);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (_la == CypherParser::SP) {
      setState(1473);
      match(CypherParser::SP);
    }
    setState(1509);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if ((((_la & ~ 0x3fULL) == 0) &&
      ((1ULL << _la) & ((1ULL << CypherParser::UNION)
      | (1ULL << CypherParser::ALL)
      | (1ULL << CypherParser::OPTIONAL)
      | (1ULL << CypherParser::MATCH)
      | (1ULL << CypherParser::UNWIND)
      | (1ULL << CypherParser::AS)
      | (1ULL << CypherParser::MERGE)
      | (1ULL << CypherParser::ON)
      | (1ULL << CypherParser::CREATE)
      | (1ULL << CypherParser::SET)
      | (1ULL << CypherParser::DETACH)
      | (1ULL << CypherParser::DELETE)
      | (1ULL << CypherParser::REMOVE)
      | (1ULL << CypherParser::WITH)
      | (1ULL << CypherParser::DISTINCT)
      | (1ULL << CypherParser::RETURN))) != 0) || ((((_la - 64) & ~ 0x3fULL) == 0) &&
      ((1ULL << (_la - 64)) & ((1ULL << (CypherParser::ORDER - 64))
      | (1ULL << (CypherParser::BY - 64))
      | (1ULL << (CypherParser::L_SKIP - 64))
      | (1ULL << (CypherParser::LIMIT - 64))
      | (1ULL << (CypherParser::ASCENDING - 64))
      | (1ULL << (CypherParser::ASC - 64))
      | (1ULL << (CypherParser::DESCENDING - 64))
      | (1ULL << (CypherParser::DESC - 64))
      | (1ULL << (CypherParser::WHERE - 64))
      | (1ULL << (CypherParser::OR - 64))
      | (1ULL << (CypherParser::XOR - 64))
      | (1ULL << (CypherParser::AND - 64))
      | (1ULL << (CypherParser::NOT - 64))
      | (1ULL << (CypherParser::IN - 64))
      | (1ULL << (CypherParser::STARTS - 64))
      | (1ULL << (CypherParser::ENDS - 64))
      | (1ULL << (CypherParser::CONTAINS - 64))
      | (1ULL << (CypherParser::IS - 64))
      | (1ULL << (CypherParser::NULLOP - 64))
      | (1ULL << (CypherParser::COUNT - 64))
      | (1ULL << (CypherParser::ANY - 64))
      | (1ULL << (CypherParser::NONE - 64))
      | (1ULL << (CypherParser::SINGLE - 64))
      | (1ULL << (CypherParser::TRUE - 64))
      | (1ULL << (CypherParser::FALSE - 64))
      | (1ULL << (CypherParser::EXISTS - 64))
      | (1ULL << (CypherParser::CASE - 64))
      | (1ULL << (CypherParser::ELSE - 64))
      | (1ULL << (CypherParser::END - 64))
      | (1ULL << (CypherParser::WHEN - 64))
      | (1ULL << (CypherParser::THEN - 64))
      | (1ULL << (CypherParser::HexLetter - 64))
      | (1ULL << (CypherParser::CONSTRAINT - 64))
      | (1ULL << (CypherParser::DO - 64))
      | (1ULL << (CypherParser::FOR - 64))
      | (1ULL << (CypherParser::REQUIRE - 64))
      | (1ULL << (CypherParser::UNIQUE - 64))
      | (1ULL << (CypherParser::MANDATORY - 64))
      | (1ULL << (CypherParser::SCALAR - 64))
      | (1ULL << (CypherParser::OF - 64))
      | (1ULL << (CypherParser::ADD - 64))
      | (1ULL << (CypherParser::DROP - 64))
      | (1ULL << (CypherParser::FILTER - 64))
      | (1ULL << (CypherParser::EXTRACT - 64))
      | (1ULL << (CypherParser::UnescapedSymbolicName - 64))
      | (1ULL << (CypherParser::EscapedSymbolicName - 64)))) != 0)) {
      setState(1476);
      oC_PropertyKeyName();
      setState(1478);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(1477);
        match(CypherParser::SP);
      }
      setState(1480);
      match(CypherParser::T__9);
      setState(1482);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(1481);
        match(CypherParser::SP);
      }
      setState(1484);
      oC_Expression();
      setState(1486);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == CypherParser::SP) {
        setState(1485);
        match(CypherParser::SP);
      }
      setState(1506);
      _errHandler->sync(this);
      _la = _input->LA(1);
      while (_la == CypherParser::T__1) {
        setState(1488);
        match(CypherParser::T__1);
        setState(1490);
        _errHandler->sync(this);

        _la = _input->LA(1);
        if (_la == CypherParser::SP) {
          setState(1489);
          match(CypherParser::SP);
        }
        setState(1492);
        oC_PropertyKeyName();
        setState(1494);
        _errHandler->sync(this);

        _la = _input->LA(1);
        if (_la == CypherParser::SP) {
          setState(1493);
          match(CypherParser::SP);
        }
        setState(1496);
        match(CypherParser::T__9);
        setState(1498);
        _errHandler->sync(this);

        _la = _input->LA(1);
        if (_la == CypherParser::SP) {
          setState(1497);
          match(CypherParser::SP);
        }
        setState(1500);
        oC_Expression();
        setState(1502);
        _errHandler->sync(this);

        _la = _input->LA(1);
        if (_la == CypherParser::SP) {
          setState(1501);
          match(CypherParser::SP);
        }
        setState(1508);
        _errHandler->sync(this);
        _la = _input->LA(1);
      }
    }
    setState(1511);
    match(CypherParser::T__24);
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_ParameterContext ------------------------------------------------------------------

CypherParser::OC_ParameterContext::OC_ParameterContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

CypherParser::OC_SymbolicNameContext* CypherParser::OC_ParameterContext::oC_SymbolicName() {
  return getRuleContext<CypherParser::OC_SymbolicNameContext>(0);
}

tree::TerminalNode* CypherParser::OC_ParameterContext::DecimalInteger() {
  return getToken(CypherParser::DecimalInteger, 0);
}


size_t CypherParser::OC_ParameterContext::getRuleIndex() const {
  return CypherParser::RuleOC_Parameter;
}

void CypherParser::OC_ParameterContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_Parameter(this);
}

void CypherParser::OC_ParameterContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_Parameter(this);
}

CypherParser::OC_ParameterContext* CypherParser::oC_Parameter() {
  OC_ParameterContext *_localctx = _tracker.createInstance<OC_ParameterContext>(_ctx, getState());
  enterRule(_localctx, 176, CypherParser::RuleOC_Parameter);

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(1513);
    match(CypherParser::T__25);
    setState(1516);
    _errHandler->sync(this);
    switch (_input->LA(1)) {
      case CypherParser::COUNT:
      case CypherParser::ANY:
      case CypherParser::NONE:
      case CypherParser::SINGLE:
      case CypherParser::HexLetter:
      case CypherParser::FILTER:
      case CypherParser::EXTRACT:
      case CypherParser::UnescapedSymbolicName:
      case CypherParser::EscapedSymbolicName: {
        setState(1514);
        oC_SymbolicName();
        break;
      }

      case CypherParser::DecimalInteger: {
        setState(1515);
        match(CypherParser::DecimalInteger);
        break;
      }

    default:
      throw NoViableAltException(this);
    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_PropertyExpressionContext ------------------------------------------------------------------

CypherParser::OC_PropertyExpressionContext::OC_PropertyExpressionContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

CypherParser::OC_AtomContext* CypherParser::OC_PropertyExpressionContext::oC_Atom() {
  return getRuleContext<CypherParser::OC_AtomContext>(0);
}

std::vector<CypherParser::OC_PropertyLookupContext *> CypherParser::OC_PropertyExpressionContext::oC_PropertyLookup() {
  return getRuleContexts<CypherParser::OC_PropertyLookupContext>();
}

CypherParser::OC_PropertyLookupContext* CypherParser::OC_PropertyExpressionContext::oC_PropertyLookup(size_t i) {
  return getRuleContext<CypherParser::OC_PropertyLookupContext>(i);
}

std::vector<tree::TerminalNode *> CypherParser::OC_PropertyExpressionContext::SP() {
  return getTokens(CypherParser::SP);
}

tree::TerminalNode* CypherParser::OC_PropertyExpressionContext::SP(size_t i) {
  return getToken(CypherParser::SP, i);
}


size_t CypherParser::OC_PropertyExpressionContext::getRuleIndex() const {
  return CypherParser::RuleOC_PropertyExpression;
}

void CypherParser::OC_PropertyExpressionContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_PropertyExpression(this);
}

void CypherParser::OC_PropertyExpressionContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_PropertyExpression(this);
}

CypherParser::OC_PropertyExpressionContext* CypherParser::oC_PropertyExpression() {
  OC_PropertyExpressionContext *_localctx = _tracker.createInstance<OC_PropertyExpressionContext>(_ctx, getState());
  enterRule(_localctx, 178, CypherParser::RuleOC_PropertyExpression);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    size_t alt;
    enterOuterAlt(_localctx, 1);
    setState(1518);
    oC_Atom();
    setState(1523); 
    _errHandler->sync(this);
    alt = 1;
    do {
      switch (alt) {
        case 1: {
              setState(1520);
              _errHandler->sync(this);

              _la = _input->LA(1);
              if (_la == CypherParser::SP) {
                setState(1519);
                match(CypherParser::SP);
              }
              setState(1522);
              oC_PropertyLookup();
              break;
            }

      default:
        throw NoViableAltException(this);
      }
      setState(1525); 
      _errHandler->sync(this);
      alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 283, _ctx);
    } while (alt != 2 && alt != atn::ATN::INVALID_ALT_NUMBER);
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_PropertyKeyNameContext ------------------------------------------------------------------

CypherParser::OC_PropertyKeyNameContext::OC_PropertyKeyNameContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

CypherParser::OC_SchemaNameContext* CypherParser::OC_PropertyKeyNameContext::oC_SchemaName() {
  return getRuleContext<CypherParser::OC_SchemaNameContext>(0);
}


size_t CypherParser::OC_PropertyKeyNameContext::getRuleIndex() const {
  return CypherParser::RuleOC_PropertyKeyName;
}

void CypherParser::OC_PropertyKeyNameContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_PropertyKeyName(this);
}

void CypherParser::OC_PropertyKeyNameContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_PropertyKeyName(this);
}

CypherParser::OC_PropertyKeyNameContext* CypherParser::oC_PropertyKeyName() {
  OC_PropertyKeyNameContext *_localctx = _tracker.createInstance<OC_PropertyKeyNameContext>(_ctx, getState());
  enterRule(_localctx, 180, CypherParser::RuleOC_PropertyKeyName);

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(1527);
    oC_SchemaName();
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_IntegerLiteralContext ------------------------------------------------------------------

CypherParser::OC_IntegerLiteralContext::OC_IntegerLiteralContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

tree::TerminalNode* CypherParser::OC_IntegerLiteralContext::HexInteger() {
  return getToken(CypherParser::HexInteger, 0);
}

tree::TerminalNode* CypherParser::OC_IntegerLiteralContext::OctalInteger() {
  return getToken(CypherParser::OctalInteger, 0);
}

tree::TerminalNode* CypherParser::OC_IntegerLiteralContext::DecimalInteger() {
  return getToken(CypherParser::DecimalInteger, 0);
}


size_t CypherParser::OC_IntegerLiteralContext::getRuleIndex() const {
  return CypherParser::RuleOC_IntegerLiteral;
}

void CypherParser::OC_IntegerLiteralContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_IntegerLiteral(this);
}

void CypherParser::OC_IntegerLiteralContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_IntegerLiteral(this);
}

CypherParser::OC_IntegerLiteralContext* CypherParser::oC_IntegerLiteral() {
  OC_IntegerLiteralContext *_localctx = _tracker.createInstance<OC_IntegerLiteralContext>(_ctx, getState());
  enterRule(_localctx, 182, CypherParser::RuleOC_IntegerLiteral);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(1529);
    _la = _input->LA(1);
    if (!(((((_la - 97) & ~ 0x3fULL) == 0) &&
      ((1ULL << (_la - 97)) & ((1ULL << (CypherParser::HexInteger - 97))
      | (1ULL << (CypherParser::DecimalInteger - 97))
      | (1ULL << (CypherParser::OctalInteger - 97)))) != 0))) {
    _errHandler->recoverInline(this);
    }
    else {
      _errHandler->reportMatch(this);
      consume();
    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_DoubleLiteralContext ------------------------------------------------------------------

CypherParser::OC_DoubleLiteralContext::OC_DoubleLiteralContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

tree::TerminalNode* CypherParser::OC_DoubleLiteralContext::ExponentDecimalReal() {
  return getToken(CypherParser::ExponentDecimalReal, 0);
}

tree::TerminalNode* CypherParser::OC_DoubleLiteralContext::RegularDecimalReal() {
  return getToken(CypherParser::RegularDecimalReal, 0);
}


size_t CypherParser::OC_DoubleLiteralContext::getRuleIndex() const {
  return CypherParser::RuleOC_DoubleLiteral;
}

void CypherParser::OC_DoubleLiteralContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_DoubleLiteral(this);
}

void CypherParser::OC_DoubleLiteralContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_DoubleLiteral(this);
}

CypherParser::OC_DoubleLiteralContext* CypherParser::oC_DoubleLiteral() {
  OC_DoubleLiteralContext *_localctx = _tracker.createInstance<OC_DoubleLiteralContext>(_ctx, getState());
  enterRule(_localctx, 184, CypherParser::RuleOC_DoubleLiteral);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(1531);
    _la = _input->LA(1);
    if (!(_la == CypherParser::ExponentDecimalReal

    || _la == CypherParser::RegularDecimalReal)) {
    _errHandler->recoverInline(this);
    }
    else {
      _errHandler->reportMatch(this);
      consume();
    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_SchemaNameContext ------------------------------------------------------------------

CypherParser::OC_SchemaNameContext::OC_SchemaNameContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

CypherParser::OC_SymbolicNameContext* CypherParser::OC_SchemaNameContext::oC_SymbolicName() {
  return getRuleContext<CypherParser::OC_SymbolicNameContext>(0);
}

CypherParser::OC_ReservedWordContext* CypherParser::OC_SchemaNameContext::oC_ReservedWord() {
  return getRuleContext<CypherParser::OC_ReservedWordContext>(0);
}


size_t CypherParser::OC_SchemaNameContext::getRuleIndex() const {
  return CypherParser::RuleOC_SchemaName;
}

void CypherParser::OC_SchemaNameContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_SchemaName(this);
}

void CypherParser::OC_SchemaNameContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_SchemaName(this);
}

CypherParser::OC_SchemaNameContext* CypherParser::oC_SchemaName() {
  OC_SchemaNameContext *_localctx = _tracker.createInstance<OC_SchemaNameContext>(_ctx, getState());
  enterRule(_localctx, 186, CypherParser::RuleOC_SchemaName);

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    setState(1535);
    _errHandler->sync(this);
    switch (_input->LA(1)) {
      case CypherParser::COUNT:
      case CypherParser::ANY:
      case CypherParser::NONE:
      case CypherParser::SINGLE:
      case CypherParser::HexLetter:
      case CypherParser::FILTER:
      case CypherParser::EXTRACT:
      case CypherParser::UnescapedSymbolicName:
      case CypherParser::EscapedSymbolicName: {
        enterOuterAlt(_localctx, 1);
        setState(1533);
        oC_SymbolicName();
        break;
      }

      case CypherParser::UNION:
      case CypherParser::ALL:
      case CypherParser::OPTIONAL:
      case CypherParser::MATCH:
      case CypherParser::UNWIND:
      case CypherParser::AS:
      case CypherParser::MERGE:
      case CypherParser::ON:
      case CypherParser::CREATE:
      case CypherParser::SET:
      case CypherParser::DETACH:
      case CypherParser::DELETE:
      case CypherParser::REMOVE:
      case CypherParser::WITH:
      case CypherParser::DISTINCT:
      case CypherParser::RETURN:
      case CypherParser::ORDER:
      case CypherParser::BY:
      case CypherParser::L_SKIP:
      case CypherParser::LIMIT:
      case CypherParser::ASCENDING:
      case CypherParser::ASC:
      case CypherParser::DESCENDING:
      case CypherParser::DESC:
      case CypherParser::WHERE:
      case CypherParser::OR:
      case CypherParser::XOR:
      case CypherParser::AND:
      case CypherParser::NOT:
      case CypherParser::IN:
      case CypherParser::STARTS:
      case CypherParser::ENDS:
      case CypherParser::CONTAINS:
      case CypherParser::IS:
      case CypherParser::NULLOP:
      case CypherParser::TRUE:
      case CypherParser::FALSE:
      case CypherParser::EXISTS:
      case CypherParser::CASE:
      case CypherParser::ELSE:
      case CypherParser::END:
      case CypherParser::WHEN:
      case CypherParser::THEN:
      case CypherParser::CONSTRAINT:
      case CypherParser::DO:
      case CypherParser::FOR:
      case CypherParser::REQUIRE:
      case CypherParser::UNIQUE:
      case CypherParser::MANDATORY:
      case CypherParser::SCALAR:
      case CypherParser::OF:
      case CypherParser::ADD:
      case CypherParser::DROP: {
        enterOuterAlt(_localctx, 2);
        setState(1534);
        oC_ReservedWord();
        break;
      }

    default:
      throw NoViableAltException(this);
    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_ReservedWordContext ------------------------------------------------------------------

CypherParser::OC_ReservedWordContext::OC_ReservedWordContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

tree::TerminalNode* CypherParser::OC_ReservedWordContext::ALL() {
  return getToken(CypherParser::ALL, 0);
}

tree::TerminalNode* CypherParser::OC_ReservedWordContext::ASC() {
  return getToken(CypherParser::ASC, 0);
}

tree::TerminalNode* CypherParser::OC_ReservedWordContext::ASCENDING() {
  return getToken(CypherParser::ASCENDING, 0);
}

tree::TerminalNode* CypherParser::OC_ReservedWordContext::BY() {
  return getToken(CypherParser::BY, 0);
}

tree::TerminalNode* CypherParser::OC_ReservedWordContext::CREATE() {
  return getToken(CypherParser::CREATE, 0);
}

tree::TerminalNode* CypherParser::OC_ReservedWordContext::DELETE() {
  return getToken(CypherParser::DELETE, 0);
}

tree::TerminalNode* CypherParser::OC_ReservedWordContext::DESC() {
  return getToken(CypherParser::DESC, 0);
}

tree::TerminalNode* CypherParser::OC_ReservedWordContext::DESCENDING() {
  return getToken(CypherParser::DESCENDING, 0);
}

tree::TerminalNode* CypherParser::OC_ReservedWordContext::DETACH() {
  return getToken(CypherParser::DETACH, 0);
}

tree::TerminalNode* CypherParser::OC_ReservedWordContext::EXISTS() {
  return getToken(CypherParser::EXISTS, 0);
}

tree::TerminalNode* CypherParser::OC_ReservedWordContext::LIMIT() {
  return getToken(CypherParser::LIMIT, 0);
}

tree::TerminalNode* CypherParser::OC_ReservedWordContext::MATCH() {
  return getToken(CypherParser::MATCH, 0);
}

tree::TerminalNode* CypherParser::OC_ReservedWordContext::MERGE() {
  return getToken(CypherParser::MERGE, 0);
}

tree::TerminalNode* CypherParser::OC_ReservedWordContext::ON() {
  return getToken(CypherParser::ON, 0);
}

tree::TerminalNode* CypherParser::OC_ReservedWordContext::OPTIONAL() {
  return getToken(CypherParser::OPTIONAL, 0);
}

tree::TerminalNode* CypherParser::OC_ReservedWordContext::ORDER() {
  return getToken(CypherParser::ORDER, 0);
}

tree::TerminalNode* CypherParser::OC_ReservedWordContext::REMOVE() {
  return getToken(CypherParser::REMOVE, 0);
}

tree::TerminalNode* CypherParser::OC_ReservedWordContext::RETURN() {
  return getToken(CypherParser::RETURN, 0);
}

tree::TerminalNode* CypherParser::OC_ReservedWordContext::SET() {
  return getToken(CypherParser::SET, 0);
}

tree::TerminalNode* CypherParser::OC_ReservedWordContext::L_SKIP() {
  return getToken(CypherParser::L_SKIP, 0);
}

tree::TerminalNode* CypherParser::OC_ReservedWordContext::WHERE() {
  return getToken(CypherParser::WHERE, 0);
}

tree::TerminalNode* CypherParser::OC_ReservedWordContext::WITH() {
  return getToken(CypherParser::WITH, 0);
}

tree::TerminalNode* CypherParser::OC_ReservedWordContext::UNION() {
  return getToken(CypherParser::UNION, 0);
}

tree::TerminalNode* CypherParser::OC_ReservedWordContext::UNWIND() {
  return getToken(CypherParser::UNWIND, 0);
}

tree::TerminalNode* CypherParser::OC_ReservedWordContext::AND() {
  return getToken(CypherParser::AND, 0);
}

tree::TerminalNode* CypherParser::OC_ReservedWordContext::AS() {
  return getToken(CypherParser::AS, 0);
}

tree::TerminalNode* CypherParser::OC_ReservedWordContext::CONTAINS() {
  return getToken(CypherParser::CONTAINS, 0);
}

tree::TerminalNode* CypherParser::OC_ReservedWordContext::DISTINCT() {
  return getToken(CypherParser::DISTINCT, 0);
}

tree::TerminalNode* CypherParser::OC_ReservedWordContext::ENDS() {
  return getToken(CypherParser::ENDS, 0);
}

tree::TerminalNode* CypherParser::OC_ReservedWordContext::IN() {
  return getToken(CypherParser::IN, 0);
}

tree::TerminalNode* CypherParser::OC_ReservedWordContext::IS() {
  return getToken(CypherParser::IS, 0);
}

tree::TerminalNode* CypherParser::OC_ReservedWordContext::NOT() {
  return getToken(CypherParser::NOT, 0);
}

tree::TerminalNode* CypherParser::OC_ReservedWordContext::OR() {
  return getToken(CypherParser::OR, 0);
}

tree::TerminalNode* CypherParser::OC_ReservedWordContext::STARTS() {
  return getToken(CypherParser::STARTS, 0);
}

tree::TerminalNode* CypherParser::OC_ReservedWordContext::XOR() {
  return getToken(CypherParser::XOR, 0);
}

tree::TerminalNode* CypherParser::OC_ReservedWordContext::FALSE() {
  return getToken(CypherParser::FALSE, 0);
}

tree::TerminalNode* CypherParser::OC_ReservedWordContext::TRUE() {
  return getToken(CypherParser::TRUE, 0);
}

tree::TerminalNode* CypherParser::OC_ReservedWordContext::NULLOP() {
  return getToken(CypherParser::NULLOP, 0);
}

tree::TerminalNode* CypherParser::OC_ReservedWordContext::CONSTRAINT() {
  return getToken(CypherParser::CONSTRAINT, 0);
}

tree::TerminalNode* CypherParser::OC_ReservedWordContext::DO() {
  return getToken(CypherParser::DO, 0);
}

tree::TerminalNode* CypherParser::OC_ReservedWordContext::FOR() {
  return getToken(CypherParser::FOR, 0);
}

tree::TerminalNode* CypherParser::OC_ReservedWordContext::REQUIRE() {
  return getToken(CypherParser::REQUIRE, 0);
}

tree::TerminalNode* CypherParser::OC_ReservedWordContext::UNIQUE() {
  return getToken(CypherParser::UNIQUE, 0);
}

tree::TerminalNode* CypherParser::OC_ReservedWordContext::CASE() {
  return getToken(CypherParser::CASE, 0);
}

tree::TerminalNode* CypherParser::OC_ReservedWordContext::WHEN() {
  return getToken(CypherParser::WHEN, 0);
}

tree::TerminalNode* CypherParser::OC_ReservedWordContext::THEN() {
  return getToken(CypherParser::THEN, 0);
}

tree::TerminalNode* CypherParser::OC_ReservedWordContext::ELSE() {
  return getToken(CypherParser::ELSE, 0);
}

tree::TerminalNode* CypherParser::OC_ReservedWordContext::END() {
  return getToken(CypherParser::END, 0);
}

tree::TerminalNode* CypherParser::OC_ReservedWordContext::MANDATORY() {
  return getToken(CypherParser::MANDATORY, 0);
}

tree::TerminalNode* CypherParser::OC_ReservedWordContext::SCALAR() {
  return getToken(CypherParser::SCALAR, 0);
}

tree::TerminalNode* CypherParser::OC_ReservedWordContext::OF() {
  return getToken(CypherParser::OF, 0);
}

tree::TerminalNode* CypherParser::OC_ReservedWordContext::ADD() {
  return getToken(CypherParser::ADD, 0);
}

tree::TerminalNode* CypherParser::OC_ReservedWordContext::DROP() {
  return getToken(CypherParser::DROP, 0);
}


size_t CypherParser::OC_ReservedWordContext::getRuleIndex() const {
  return CypherParser::RuleOC_ReservedWord;
}

void CypherParser::OC_ReservedWordContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_ReservedWord(this);
}

void CypherParser::OC_ReservedWordContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_ReservedWord(this);
}

CypherParser::OC_ReservedWordContext* CypherParser::oC_ReservedWord() {
  OC_ReservedWordContext *_localctx = _tracker.createInstance<OC_ReservedWordContext>(_ctx, getState());
  enterRule(_localctx, 188, CypherParser::RuleOC_ReservedWord);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(1537);
    _la = _input->LA(1);
    if (!((((_la & ~ 0x3fULL) == 0) &&
      ((1ULL << _la) & ((1ULL << CypherParser::UNION)
      | (1ULL << CypherParser::ALL)
      | (1ULL << CypherParser::OPTIONAL)
      | (1ULL << CypherParser::MATCH)
      | (1ULL << CypherParser::UNWIND)
      | (1ULL << CypherParser::AS)
      | (1ULL << CypherParser::MERGE)
      | (1ULL << CypherParser::ON)
      | (1ULL << CypherParser::CREATE)
      | (1ULL << CypherParser::SET)
      | (1ULL << CypherParser::DETACH)
      | (1ULL << CypherParser::DELETE)
      | (1ULL << CypherParser::REMOVE)
      | (1ULL << CypherParser::WITH)
      | (1ULL << CypherParser::DISTINCT)
      | (1ULL << CypherParser::RETURN))) != 0) || ((((_la - 64) & ~ 0x3fULL) == 0) &&
      ((1ULL << (_la - 64)) & ((1ULL << (CypherParser::ORDER - 64))
      | (1ULL << (CypherParser::BY - 64))
      | (1ULL << (CypherParser::L_SKIP - 64))
      | (1ULL << (CypherParser::LIMIT - 64))
      | (1ULL << (CypherParser::ASCENDING - 64))
      | (1ULL << (CypherParser::ASC - 64))
      | (1ULL << (CypherParser::DESCENDING - 64))
      | (1ULL << (CypherParser::DESC - 64))
      | (1ULL << (CypherParser::WHERE - 64))
      | (1ULL << (CypherParser::OR - 64))
      | (1ULL << (CypherParser::XOR - 64))
      | (1ULL << (CypherParser::AND - 64))
      | (1ULL << (CypherParser::NOT - 64))
      | (1ULL << (CypherParser::IN - 64))
      | (1ULL << (CypherParser::STARTS - 64))
      | (1ULL << (CypherParser::ENDS - 64))
      | (1ULL << (CypherParser::CONTAINS - 64))
      | (1ULL << (CypherParser::IS - 64))
      | (1ULL << (CypherParser::NULLOP - 64))
      | (1ULL << (CypherParser::TRUE - 64))
      | (1ULL << (CypherParser::FALSE - 64))
      | (1ULL << (CypherParser::EXISTS - 64))
      | (1ULL << (CypherParser::CASE - 64))
      | (1ULL << (CypherParser::ELSE - 64))
      | (1ULL << (CypherParser::END - 64))
      | (1ULL << (CypherParser::WHEN - 64))
      | (1ULL << (CypherParser::THEN - 64))
      | (1ULL << (CypherParser::CONSTRAINT - 64))
      | (1ULL << (CypherParser::DO - 64))
      | (1ULL << (CypherParser::FOR - 64))
      | (1ULL << (CypherParser::REQUIRE - 64))
      | (1ULL << (CypherParser::UNIQUE - 64))
      | (1ULL << (CypherParser::MANDATORY - 64))
      | (1ULL << (CypherParser::SCALAR - 64))
      | (1ULL << (CypherParser::OF - 64))
      | (1ULL << (CypherParser::ADD - 64))
      | (1ULL << (CypherParser::DROP - 64)))) != 0))) {
    _errHandler->recoverInline(this);
    }
    else {
      _errHandler->reportMatch(this);
      consume();
    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_SymbolicNameContext ------------------------------------------------------------------

CypherParser::OC_SymbolicNameContext::OC_SymbolicNameContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

tree::TerminalNode* CypherParser::OC_SymbolicNameContext::UnescapedSymbolicName() {
  return getToken(CypherParser::UnescapedSymbolicName, 0);
}

tree::TerminalNode* CypherParser::OC_SymbolicNameContext::EscapedSymbolicName() {
  return getToken(CypherParser::EscapedSymbolicName, 0);
}

tree::TerminalNode* CypherParser::OC_SymbolicNameContext::HexLetter() {
  return getToken(CypherParser::HexLetter, 0);
}

tree::TerminalNode* CypherParser::OC_SymbolicNameContext::COUNT() {
  return getToken(CypherParser::COUNT, 0);
}

tree::TerminalNode* CypherParser::OC_SymbolicNameContext::FILTER() {
  return getToken(CypherParser::FILTER, 0);
}

tree::TerminalNode* CypherParser::OC_SymbolicNameContext::EXTRACT() {
  return getToken(CypherParser::EXTRACT, 0);
}

tree::TerminalNode* CypherParser::OC_SymbolicNameContext::ANY() {
  return getToken(CypherParser::ANY, 0);
}

tree::TerminalNode* CypherParser::OC_SymbolicNameContext::NONE() {
  return getToken(CypherParser::NONE, 0);
}

tree::TerminalNode* CypherParser::OC_SymbolicNameContext::SINGLE() {
  return getToken(CypherParser::SINGLE, 0);
}


size_t CypherParser::OC_SymbolicNameContext::getRuleIndex() const {
  return CypherParser::RuleOC_SymbolicName;
}

void CypherParser::OC_SymbolicNameContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_SymbolicName(this);
}

void CypherParser::OC_SymbolicNameContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_SymbolicName(this);
}

CypherParser::OC_SymbolicNameContext* CypherParser::oC_SymbolicName() {
  OC_SymbolicNameContext *_localctx = _tracker.createInstance<OC_SymbolicNameContext>(_ctx, getState());
  enterRule(_localctx, 190, CypherParser::RuleOC_SymbolicName);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(1539);
    _la = _input->LA(1);
    if (!(((((_la - 83) & ~ 0x3fULL) == 0) &&
      ((1ULL << (_la - 83)) & ((1ULL << (CypherParser::COUNT - 83))
      | (1ULL << (CypherParser::ANY - 83))
      | (1ULL << (CypherParser::NONE - 83))
      | (1ULL << (CypherParser::SINGLE - 83))
      | (1ULL << (CypherParser::HexLetter - 83))
      | (1ULL << (CypherParser::FILTER - 83))
      | (1ULL << (CypherParser::EXTRACT - 83))
      | (1ULL << (CypherParser::UnescapedSymbolicName - 83))
      | (1ULL << (CypherParser::EscapedSymbolicName - 83)))) != 0))) {
    _errHandler->recoverInline(this);
    }
    else {
      _errHandler->reportMatch(this);
      consume();
    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_LeftArrowHeadContext ------------------------------------------------------------------

CypherParser::OC_LeftArrowHeadContext::OC_LeftArrowHeadContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}


size_t CypherParser::OC_LeftArrowHeadContext::getRuleIndex() const {
  return CypherParser::RuleOC_LeftArrowHead;
}

void CypherParser::OC_LeftArrowHeadContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_LeftArrowHead(this);
}

void CypherParser::OC_LeftArrowHeadContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_LeftArrowHead(this);
}

CypherParser::OC_LeftArrowHeadContext* CypherParser::oC_LeftArrowHead() {
  OC_LeftArrowHeadContext *_localctx = _tracker.createInstance<OC_LeftArrowHeadContext>(_ctx, getState());
  enterRule(_localctx, 192, CypherParser::RuleOC_LeftArrowHead);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(1541);
    _la = _input->LA(1);
    if (!((((_la & ~ 0x3fULL) == 0) &&
      ((1ULL << _la) & ((1ULL << CypherParser::T__18)
      | (1ULL << CypherParser::T__26)
      | (1ULL << CypherParser::T__27)
      | (1ULL << CypherParser::T__28)
      | (1ULL << CypherParser::T__29))) != 0))) {
    _errHandler->recoverInline(this);
    }
    else {
      _errHandler->reportMatch(this);
      consume();
    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_RightArrowHeadContext ------------------------------------------------------------------

CypherParser::OC_RightArrowHeadContext::OC_RightArrowHeadContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}


size_t CypherParser::OC_RightArrowHeadContext::getRuleIndex() const {
  return CypherParser::RuleOC_RightArrowHead;
}

void CypherParser::OC_RightArrowHeadContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_RightArrowHead(this);
}

void CypherParser::OC_RightArrowHeadContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_RightArrowHead(this);
}

CypherParser::OC_RightArrowHeadContext* CypherParser::oC_RightArrowHead() {
  OC_RightArrowHeadContext *_localctx = _tracker.createInstance<OC_RightArrowHeadContext>(_ctx, getState());
  enterRule(_localctx, 194, CypherParser::RuleOC_RightArrowHead);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(1543);
    _la = _input->LA(1);
    if (!((((_la & ~ 0x3fULL) == 0) &&
      ((1ULL << _la) & ((1ULL << CypherParser::T__19)
      | (1ULL << CypherParser::T__30)
      | (1ULL << CypherParser::T__31)
      | (1ULL << CypherParser::T__32)
      | (1ULL << CypherParser::T__33))) != 0))) {
    _errHandler->recoverInline(this);
    }
    else {
      _errHandler->reportMatch(this);
      consume();
    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- OC_DashContext ------------------------------------------------------------------

CypherParser::OC_DashContext::OC_DashContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}


size_t CypherParser::OC_DashContext::getRuleIndex() const {
  return CypherParser::RuleOC_Dash;
}

void CypherParser::OC_DashContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterOC_Dash(this);
}

void CypherParser::OC_DashContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<CypherListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitOC_Dash(this);
}

CypherParser::OC_DashContext* CypherParser::oC_Dash() {
  OC_DashContext *_localctx = _tracker.createInstance<OC_DashContext>(_ctx, getState());
  enterRule(_localctx, 196, CypherParser::RuleOC_Dash);
  size_t _la = 0;

  auto onExit = finally([=] {
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(1545);
    _la = _input->LA(1);
    if (!((((_la & ~ 0x3fULL) == 0) &&
      ((1ULL << _la) & ((1ULL << CypherParser::T__13)
      | (1ULL << CypherParser::T__34)
      | (1ULL << CypherParser::T__35)
      | (1ULL << CypherParser::T__36)
      | (1ULL << CypherParser::T__37)
      | (1ULL << CypherParser::T__38)
      | (1ULL << CypherParser::T__39)
      | (1ULL << CypherParser::T__40)
      | (1ULL << CypherParser::T__41)
      | (1ULL << CypherParser::T__42)
      | (1ULL << CypherParser::T__43)
      | (1ULL << CypherParser::T__44))) != 0))) {
    _errHandler->recoverInline(this);
    }
    else {
      _errHandler->reportMatch(this);
      consume();
    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

// Static vars and initialization.
std::vector<dfa::DFA> CypherParser::_decisionToDFA;
atn::PredictionContextCache CypherParser::_sharedContextCache;

// We own the ATN which in turn owns the ATN states.
atn::ATN CypherParser::_atn;
std::vector<uint16_t> CypherParser::_serializedATN;

std::vector<std::string> CypherParser::_ruleNames = {
  "oC_Cypher", "oC_Statement", "oC_Query", "oC_RegularQuery", "oC_Union", 
  "oC_SingleQuery", "oC_SinglePartQuery", "oC_MultiPartQuery", "oC_UpdatingClause", 
  "oC_ReadingClause", "oC_Match", "oC_Unwind", "oC_Merge", "oC_MergeAction", 
  "oC_Create", "oC_Set", "oC_SetItem", "oC_Delete", "oC_Remove", "oC_RemoveItem", 
  "oC_InQueryCall", "oC_StandaloneCall", "oC_YieldItems", "oC_YieldItem", 
  "oC_With", "oC_Return", "oC_ReturnBody", "oC_ReturnItems", "oC_ReturnItem", 
  "oC_Order", "oC_Skip", "oC_Limit", "oC_SortItem", "oC_Where", "oC_Pattern", 
  "oC_PatternPart", "oC_AnonymousPatternPart", "oC_PatternElement", "oC_NodePattern", 
  "oC_PatternElementChain", "oC_RelationshipPattern", "oC_RelationshipDetail", 
  "oC_Properties", "oC_RelationshipTypes", "oC_NodeLabels", "oC_NodeLabel", 
  "oC_RangeLiteral", "oC_LabelName", "oC_RelTypeName", "oC_Expression", 
  "oC_OrExpression", "oC_XorExpression", "oC_AndExpression", "oC_NotExpression", 
  "oC_ComparisonExpression", "oC_AddOrSubtractExpression", "oC_MultiplyDivideModuloExpression", 
  "oC_PowerOfExpression", "oC_UnaryAddOrSubtractExpression", "oC_StringListNullOperatorExpression", 
  "oC_ListOperatorExpression", "oC_StringOperatorExpression", "oC_NullOperatorExpression", 
  "oC_PropertyOrLabelsExpression", "oC_Atom", "oC_Literal", "oC_BooleanLiteral", 
  "oC_ListLiteral", "oC_PartialComparisonExpression", "oC_ParenthesizedExpression", 
  "oC_RelationshipsPattern", "oC_FilterExpression", "oC_IdInColl", "oC_FunctionInvocation", 
  "oC_FunctionName", "oC_ExplicitProcedureInvocation", "oC_ImplicitProcedureInvocation", 
  "oC_ProcedureResultField", "oC_ProcedureName", "oC_Namespace", "oC_ListComprehension", 
  "oC_PatternComprehension", "oC_PropertyLookup", "oC_CaseExpression", "oC_CaseAlternatives", 
  "oC_Variable", "oC_NumberLiteral", "oC_MapLiteral", "oC_Parameter", "oC_PropertyExpression", 
  "oC_PropertyKeyName", "oC_IntegerLiteral", "oC_DoubleLiteral", "oC_SchemaName", 
  "oC_ReservedWord", "oC_SymbolicName", "oC_LeftArrowHead", "oC_RightArrowHead", 
  "oC_Dash"
};

std::vector<std::string> CypherParser::_literalNames = {
  "", "';'", "','", "'='", "'+='", "'*'", "'('", "')'", "'['", "']'", "':'", 
  "'|'", "'..'", "'+'", "'-'", "'/'", "'%'", "'^'", "'<>'", "'<'", "'>'", 
  "'<='", "'>='", "'.'", "'{'", "'}'", "'$'", "'\u00E2\u0178\u00A8'", "'\u00E3\u20AC\u02C6'", 
  "'\u00EF\u00B9\u00A4'", "'\u00EF\u00BC\u0153'", "'\u00E2\u0178\u00A9'", 
  "'\u00E3\u20AC\u2030'", "'\u00EF\u00B9\u00A5'", "'\u00EF\u00BC\u017E'", 
  "'\u00C2\u00AD'", "'\u00E2\u20AC\uFFFD'", "'\u00E2\u20AC\u2018'", "'\u00E2\u20AC\u2019'", 
  "'\u00E2\u20AC\u201C'", "'\u00E2\u20AC\u201D'", "'\u00E2\u20AC\u2022'", 
  "'\u00E2\u02C6\u2019'", "'\u00EF\u00B9\u02DC'", "'\u00EF\u00B9\u00A3'", 
  "'\u00EF\u00BC\uFFFD'", "", "", "", "", "", "", "", "", "", "", "", "", 
  "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", 
  "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", 
  "", "", "", "", "", "", "", "", "", "", "", "", "'0'"
};

std::vector<std::string> CypherParser::_symbolicNames = {
  "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", 
  "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", 
  "", "", "", "", "", "", "", "", "", "", "UNION", "ALL", "OPTIONAL", "MATCH", 
  "UNWIND", "AS", "MERGE", "ON", "CREATE", "SET", "DETACH", "DELETE", "REMOVE", 
  "CALL", "YIELD", "WITH", "DISTINCT", "RETURN", "ORDER", "BY", "L_SKIP", 
  "LIMIT", "ASCENDING", "ASC", "DESCENDING", "DESC", "WHERE", "OR", "XOR", 
  "AND", "NOT", "IN", "STARTS", "ENDS", "CONTAINS", "IS", "NULLOP", "COUNT", 
  "ANY", "NONE", "SINGLE", "TRUE", "FALSE", "EXISTS", "CASE", "ELSE", "END", 
  "WHEN", "THEN", "StringLiteral", "EscapedChar", "HexInteger", "DecimalInteger", 
  "OctalInteger", "HexLetter", "HexDigit", "Digit", "NonZeroDigit", "NonZeroOctDigit", 
  "OctDigit", "ZeroDigit", "ExponentDecimalReal", "RegularDecimalReal", 
  "CONSTRAINT", "DO", "FOR", "REQUIRE", "UNIQUE", "MANDATORY", "SCALAR", 
  "OF", "ADD", "DROP", "FILTER", "EXTRACT", "UnescapedSymbolicName", "IdentifierStart", 
  "IdentifierPart", "EscapedSymbolicName", "SP", "WHITESPACE", "Comment"
};

dfa::Vocabulary CypherParser::_vocabulary(_literalNames, _symbolicNames);

std::vector<std::string> CypherParser::_tokenNames;

CypherParser::Initializer::Initializer() {
	for (size_t i = 0; i < _symbolicNames.size(); ++i) {
		std::string name = _vocabulary.getLiteralName(i);
		if (name.empty()) {
			name = _vocabulary.getSymbolicName(i);
		}

		if (name.empty()) {
			_tokenNames.push_back("<INVALID>");
		} else {
      _tokenNames.push_back(name);
    }
	}

  _serializedATN = {
    0x3, 0x608b, 0xa72a, 0x8133, 0xb9ed, 0x417c, 0x3be7, 0x7786, 0x5964, 
    0x3, 0x81, 0x60e, 0x4, 0x2, 0x9, 0x2, 0x4, 0x3, 0x9, 0x3, 0x4, 0x4, 
    0x9, 0x4, 0x4, 0x5, 0x9, 0x5, 0x4, 0x6, 0x9, 0x6, 0x4, 0x7, 0x9, 0x7, 
    0x4, 0x8, 0x9, 0x8, 0x4, 0x9, 0x9, 0x9, 0x4, 0xa, 0x9, 0xa, 0x4, 0xb, 
    0x9, 0xb, 0x4, 0xc, 0x9, 0xc, 0x4, 0xd, 0x9, 0xd, 0x4, 0xe, 0x9, 0xe, 
    0x4, 0xf, 0x9, 0xf, 0x4, 0x10, 0x9, 0x10, 0x4, 0x11, 0x9, 0x11, 0x4, 
    0x12, 0x9, 0x12, 0x4, 0x13, 0x9, 0x13, 0x4, 0x14, 0x9, 0x14, 0x4, 0x15, 
    0x9, 0x15, 0x4, 0x16, 0x9, 0x16, 0x4, 0x17, 0x9, 0x17, 0x4, 0x18, 0x9, 
    0x18, 0x4, 0x19, 0x9, 0x19, 0x4, 0x1a, 0x9, 0x1a, 0x4, 0x1b, 0x9, 0x1b, 
    0x4, 0x1c, 0x9, 0x1c, 0x4, 0x1d, 0x9, 0x1d, 0x4, 0x1e, 0x9, 0x1e, 0x4, 
    0x1f, 0x9, 0x1f, 0x4, 0x20, 0x9, 0x20, 0x4, 0x21, 0x9, 0x21, 0x4, 0x22, 
    0x9, 0x22, 0x4, 0x23, 0x9, 0x23, 0x4, 0x24, 0x9, 0x24, 0x4, 0x25, 0x9, 
    0x25, 0x4, 0x26, 0x9, 0x26, 0x4, 0x27, 0x9, 0x27, 0x4, 0x28, 0x9, 0x28, 
    0x4, 0x29, 0x9, 0x29, 0x4, 0x2a, 0x9, 0x2a, 0x4, 0x2b, 0x9, 0x2b, 0x4, 
    0x2c, 0x9, 0x2c, 0x4, 0x2d, 0x9, 0x2d, 0x4, 0x2e, 0x9, 0x2e, 0x4, 0x2f, 
    0x9, 0x2f, 0x4, 0x30, 0x9, 0x30, 0x4, 0x31, 0x9, 0x31, 0x4, 0x32, 0x9, 
    0x32, 0x4, 0x33, 0x9, 0x33, 0x4, 0x34, 0x9, 0x34, 0x4, 0x35, 0x9, 0x35, 
    0x4, 0x36, 0x9, 0x36, 0x4, 0x37, 0x9, 0x37, 0x4, 0x38, 0x9, 0x38, 0x4, 
    0x39, 0x9, 0x39, 0x4, 0x3a, 0x9, 0x3a, 0x4, 0x3b, 0x9, 0x3b, 0x4, 0x3c, 
    0x9, 0x3c, 0x4, 0x3d, 0x9, 0x3d, 0x4, 0x3e, 0x9, 0x3e, 0x4, 0x3f, 0x9, 
    0x3f, 0x4, 0x40, 0x9, 0x40, 0x4, 0x41, 0x9, 0x41, 0x4, 0x42, 0x9, 0x42, 
    0x4, 0x43, 0x9, 0x43, 0x4, 0x44, 0x9, 0x44, 0x4, 0x45, 0x9, 0x45, 0x4, 
    0x46, 0x9, 0x46, 0x4, 0x47, 0x9, 0x47, 0x4, 0x48, 0x9, 0x48, 0x4, 0x49, 
    0x9, 0x49, 0x4, 0x4a, 0x9, 0x4a, 0x4, 0x4b, 0x9, 0x4b, 0x4, 0x4c, 0x9, 
    0x4c, 0x4, 0x4d, 0x9, 0x4d, 0x4, 0x4e, 0x9, 0x4e, 0x4, 0x4f, 0x9, 0x4f, 
    0x4, 0x50, 0x9, 0x50, 0x4, 0x51, 0x9, 0x51, 0x4, 0x52, 0x9, 0x52, 0x4, 
    0x53, 0x9, 0x53, 0x4, 0x54, 0x9, 0x54, 0x4, 0x55, 0x9, 0x55, 0x4, 0x56, 
    0x9, 0x56, 0x4, 0x57, 0x9, 0x57, 0x4, 0x58, 0x9, 0x58, 0x4, 0x59, 0x9, 
    0x59, 0x4, 0x5a, 0x9, 0x5a, 0x4, 0x5b, 0x9, 0x5b, 0x4, 0x5c, 0x9, 0x5c, 
    0x4, 0x5d, 0x9, 0x5d, 0x4, 0x5e, 0x9, 0x5e, 0x4, 0x5f, 0x9, 0x5f, 0x4, 
    0x60, 0x9, 0x60, 0x4, 0x61, 0x9, 0x61, 0x4, 0x62, 0x9, 0x62, 0x4, 0x63, 
    0x9, 0x63, 0x4, 0x64, 0x9, 0x64, 0x3, 0x2, 0x5, 0x2, 0xca, 0xa, 0x2, 
    0x3, 0x2, 0x3, 0x2, 0x5, 0x2, 0xce, 0xa, 0x2, 0x3, 0x2, 0x5, 0x2, 0xd1, 
    0xa, 0x2, 0x3, 0x2, 0x5, 0x2, 0xd4, 0xa, 0x2, 0x3, 0x2, 0x3, 0x2, 0x3, 
    0x3, 0x3, 0x3, 0x3, 0x4, 0x3, 0x4, 0x5, 0x4, 0xdc, 0xa, 0x4, 0x3, 0x5, 
    0x3, 0x5, 0x5, 0x5, 0xe0, 0xa, 0x5, 0x3, 0x5, 0x7, 0x5, 0xe3, 0xa, 0x5, 
    0xc, 0x5, 0xe, 0x5, 0xe6, 0xb, 0x5, 0x3, 0x6, 0x3, 0x6, 0x3, 0x6, 0x3, 
    0x6, 0x5, 0x6, 0xec, 0xa, 0x6, 0x3, 0x6, 0x3, 0x6, 0x3, 0x6, 0x5, 0x6, 
    0xf1, 0xa, 0x6, 0x3, 0x6, 0x5, 0x6, 0xf4, 0xa, 0x6, 0x3, 0x7, 0x3, 0x7, 
    0x5, 0x7, 0xf8, 0xa, 0x7, 0x3, 0x8, 0x3, 0x8, 0x5, 0x8, 0xfc, 0xa, 0x8, 
    0x7, 0x8, 0xfe, 0xa, 0x8, 0xc, 0x8, 0xe, 0x8, 0x101, 0xb, 0x8, 0x3, 
    0x8, 0x3, 0x8, 0x3, 0x8, 0x5, 0x8, 0x106, 0xa, 0x8, 0x7, 0x8, 0x108, 
    0xa, 0x8, 0xc, 0x8, 0xe, 0x8, 0x10b, 0xb, 0x8, 0x3, 0x8, 0x3, 0x8, 0x5, 
    0x8, 0x10f, 0xa, 0x8, 0x3, 0x8, 0x7, 0x8, 0x112, 0xa, 0x8, 0xc, 0x8, 
    0xe, 0x8, 0x115, 0xb, 0x8, 0x3, 0x8, 0x5, 0x8, 0x118, 0xa, 0x8, 0x3, 
    0x8, 0x5, 0x8, 0x11b, 0xa, 0x8, 0x5, 0x8, 0x11d, 0xa, 0x8, 0x3, 0x9, 
    0x3, 0x9, 0x5, 0x9, 0x121, 0xa, 0x9, 0x7, 0x9, 0x123, 0xa, 0x9, 0xc, 
    0x9, 0xe, 0x9, 0x126, 0xb, 0x9, 0x3, 0x9, 0x3, 0x9, 0x5, 0x9, 0x12a, 
    0xa, 0x9, 0x7, 0x9, 0x12c, 0xa, 0x9, 0xc, 0x9, 0xe, 0x9, 0x12f, 0xb, 
    0x9, 0x3, 0x9, 0x3, 0x9, 0x5, 0x9, 0x133, 0xa, 0x9, 0x6, 0x9, 0x135, 
    0xa, 0x9, 0xd, 0x9, 0xe, 0x9, 0x136, 0x3, 0x9, 0x3, 0x9, 0x3, 0xa, 0x3, 
    0xa, 0x3, 0xa, 0x3, 0xa, 0x3, 0xa, 0x5, 0xa, 0x140, 0xa, 0xa, 0x3, 0xb, 
    0x3, 0xb, 0x3, 0xb, 0x5, 0xb, 0x145, 0xa, 0xb, 0x3, 0xc, 0x3, 0xc, 0x5, 
    0xc, 0x149, 0xa, 0xc, 0x3, 0xc, 0x3, 0xc, 0x5, 0xc, 0x14d, 0xa, 0xc, 
    0x3, 0xc, 0x3, 0xc, 0x5, 0xc, 0x151, 0xa, 0xc, 0x3, 0xc, 0x5, 0xc, 0x154, 
    0xa, 0xc, 0x3, 0xd, 0x3, 0xd, 0x5, 0xd, 0x158, 0xa, 0xd, 0x3, 0xd, 0x3, 
    0xd, 0x3, 0xd, 0x3, 0xd, 0x3, 0xd, 0x3, 0xd, 0x3, 0xe, 0x3, 0xe, 0x5, 
    0xe, 0x162, 0xa, 0xe, 0x3, 0xe, 0x3, 0xe, 0x3, 0xe, 0x7, 0xe, 0x167, 
    0xa, 0xe, 0xc, 0xe, 0xe, 0xe, 0x16a, 0xb, 0xe, 0x3, 0xf, 0x3, 0xf, 0x3, 
    0xf, 0x3, 0xf, 0x3, 0xf, 0x3, 0xf, 0x3, 0xf, 0x3, 0xf, 0x3, 0xf, 0x3, 
    0xf, 0x5, 0xf, 0x176, 0xa, 0xf, 0x3, 0x10, 0x3, 0x10, 0x5, 0x10, 0x17a, 
    0xa, 0x10, 0x3, 0x10, 0x3, 0x10, 0x3, 0x11, 0x3, 0x11, 0x5, 0x11, 0x180, 
    0xa, 0x11, 0x3, 0x11, 0x3, 0x11, 0x3, 0x11, 0x7, 0x11, 0x185, 0xa, 0x11, 
    0xc, 0x11, 0xe, 0x11, 0x188, 0xb, 0x11, 0x3, 0x12, 0x3, 0x12, 0x5, 0x12, 
    0x18c, 0xa, 0x12, 0x3, 0x12, 0x3, 0x12, 0x5, 0x12, 0x190, 0xa, 0x12, 
    0x3, 0x12, 0x3, 0x12, 0x3, 0x12, 0x3, 0x12, 0x5, 0x12, 0x196, 0xa, 0x12, 
    0x3, 0x12, 0x3, 0x12, 0x5, 0x12, 0x19a, 0xa, 0x12, 0x3, 0x12, 0x3, 0x12, 
    0x3, 0x12, 0x3, 0x12, 0x5, 0x12, 0x1a0, 0xa, 0x12, 0x3, 0x12, 0x3, 0x12, 
    0x5, 0x12, 0x1a4, 0xa, 0x12, 0x3, 0x12, 0x3, 0x12, 0x3, 0x12, 0x3, 0x12, 
    0x5, 0x12, 0x1aa, 0xa, 0x12, 0x3, 0x12, 0x3, 0x12, 0x5, 0x12, 0x1ae, 
    0xa, 0x12, 0x3, 0x13, 0x3, 0x13, 0x5, 0x13, 0x1b2, 0xa, 0x13, 0x3, 0x13, 
    0x3, 0x13, 0x5, 0x13, 0x1b6, 0xa, 0x13, 0x3, 0x13, 0x3, 0x13, 0x5, 0x13, 
    0x1ba, 0xa, 0x13, 0x3, 0x13, 0x3, 0x13, 0x5, 0x13, 0x1be, 0xa, 0x13, 
    0x3, 0x13, 0x7, 0x13, 0x1c1, 0xa, 0x13, 0xc, 0x13, 0xe, 0x13, 0x1c4, 
    0xb, 0x13, 0x3, 0x14, 0x3, 0x14, 0x3, 0x14, 0x3, 0x14, 0x5, 0x14, 0x1ca, 
    0xa, 0x14, 0x3, 0x14, 0x3, 0x14, 0x5, 0x14, 0x1ce, 0xa, 0x14, 0x3, 0x14, 
    0x7, 0x14, 0x1d1, 0xa, 0x14, 0xc, 0x14, 0xe, 0x14, 0x1d4, 0xb, 0x14, 
    0x3, 0x15, 0x3, 0x15, 0x3, 0x15, 0x3, 0x15, 0x5, 0x15, 0x1da, 0xa, 0x15, 
    0x3, 0x16, 0x3, 0x16, 0x3, 0x16, 0x3, 0x16, 0x5, 0x16, 0x1e0, 0xa, 0x16, 
    0x3, 0x16, 0x3, 0x16, 0x3, 0x16, 0x5, 0x16, 0x1e5, 0xa, 0x16, 0x3, 0x17, 
    0x3, 0x17, 0x3, 0x17, 0x3, 0x17, 0x5, 0x17, 0x1eb, 0xa, 0x17, 0x3, 0x17, 
    0x3, 0x17, 0x3, 0x17, 0x3, 0x17, 0x5, 0x17, 0x1f1, 0xa, 0x17, 0x3, 0x18, 
    0x3, 0x18, 0x3, 0x18, 0x5, 0x18, 0x1f6, 0xa, 0x18, 0x3, 0x18, 0x3, 0x18, 
    0x5, 0x18, 0x1fa, 0xa, 0x18, 0x3, 0x18, 0x7, 0x18, 0x1fd, 0xa, 0x18, 
    0xc, 0x18, 0xe, 0x18, 0x200, 0xb, 0x18, 0x5, 0x18, 0x202, 0xa, 0x18, 
    0x3, 0x18, 0x5, 0x18, 0x205, 0xa, 0x18, 0x3, 0x18, 0x5, 0x18, 0x208, 
    0xa, 0x18, 0x3, 0x19, 0x3, 0x19, 0x3, 0x19, 0x3, 0x19, 0x3, 0x19, 0x5, 
    0x19, 0x20f, 0xa, 0x19, 0x3, 0x19, 0x3, 0x19, 0x3, 0x1a, 0x3, 0x1a, 
    0x5, 0x1a, 0x215, 0xa, 0x1a, 0x3, 0x1a, 0x5, 0x1a, 0x218, 0xa, 0x1a, 
    0x3, 0x1a, 0x3, 0x1a, 0x3, 0x1a, 0x5, 0x1a, 0x21d, 0xa, 0x1a, 0x3, 0x1a, 
    0x5, 0x1a, 0x220, 0xa, 0x1a, 0x3, 0x1b, 0x3, 0x1b, 0x5, 0x1b, 0x224, 
    0xa, 0x1b, 0x3, 0x1b, 0x5, 0x1b, 0x227, 0xa, 0x1b, 0x3, 0x1b, 0x3, 0x1b, 
    0x3, 0x1b, 0x3, 0x1c, 0x3, 0x1c, 0x3, 0x1c, 0x5, 0x1c, 0x22f, 0xa, 0x1c, 
    0x3, 0x1c, 0x3, 0x1c, 0x5, 0x1c, 0x233, 0xa, 0x1c, 0x3, 0x1c, 0x3, 0x1c, 
    0x5, 0x1c, 0x237, 0xa, 0x1c, 0x3, 0x1d, 0x3, 0x1d, 0x5, 0x1d, 0x23b, 
    0xa, 0x1d, 0x3, 0x1d, 0x3, 0x1d, 0x5, 0x1d, 0x23f, 0xa, 0x1d, 0x3, 0x1d, 
    0x7, 0x1d, 0x242, 0xa, 0x1d, 0xc, 0x1d, 0xe, 0x1d, 0x245, 0xb, 0x1d, 
    0x3, 0x1d, 0x3, 0x1d, 0x5, 0x1d, 0x249, 0xa, 0x1d, 0x3, 0x1d, 0x3, 0x1d, 
    0x5, 0x1d, 0x24d, 0xa, 0x1d, 0x3, 0x1d, 0x7, 0x1d, 0x250, 0xa, 0x1d, 
    0xc, 0x1d, 0xe, 0x1d, 0x253, 0xb, 0x1d, 0x5, 0x1d, 0x255, 0xa, 0x1d, 
    0x3, 0x1e, 0x3, 0x1e, 0x3, 0x1e, 0x3, 0x1e, 0x3, 0x1e, 0x3, 0x1e, 0x3, 
    0x1e, 0x5, 0x1e, 0x25e, 0xa, 0x1e, 0x3, 0x1f, 0x3, 0x1f, 0x3, 0x1f, 
    0x3, 0x1f, 0x3, 0x1f, 0x3, 0x1f, 0x3, 0x1f, 0x5, 0x1f, 0x267, 0xa, 0x1f, 
    0x3, 0x1f, 0x7, 0x1f, 0x26a, 0xa, 0x1f, 0xc, 0x1f, 0xe, 0x1f, 0x26d, 
    0xb, 0x1f, 0x3, 0x20, 0x3, 0x20, 0x3, 0x20, 0x3, 0x20, 0x3, 0x21, 0x3, 
    0x21, 0x3, 0x21, 0x3, 0x21, 0x3, 0x22, 0x3, 0x22, 0x5, 0x22, 0x279, 
    0xa, 0x22, 0x3, 0x22, 0x5, 0x22, 0x27c, 0xa, 0x22, 0x3, 0x23, 0x3, 0x23, 
    0x3, 0x23, 0x3, 0x23, 0x3, 0x24, 0x3, 0x24, 0x5, 0x24, 0x284, 0xa, 0x24, 
    0x3, 0x24, 0x3, 0x24, 0x5, 0x24, 0x288, 0xa, 0x24, 0x3, 0x24, 0x7, 0x24, 
    0x28b, 0xa, 0x24, 0xc, 0x24, 0xe, 0x24, 0x28e, 0xb, 0x24, 0x3, 0x25, 
    0x3, 0x25, 0x5, 0x25, 0x292, 0xa, 0x25, 0x3, 0x25, 0x3, 0x25, 0x5, 0x25, 
    0x296, 0xa, 0x25, 0x3, 0x25, 0x3, 0x25, 0x3, 0x25, 0x5, 0x25, 0x29b, 
    0xa, 0x25, 0x3, 0x26, 0x3, 0x26, 0x3, 0x27, 0x3, 0x27, 0x5, 0x27, 0x2a1, 
    0xa, 0x27, 0x3, 0x27, 0x7, 0x27, 0x2a4, 0xa, 0x27, 0xc, 0x27, 0xe, 0x27, 
    0x2a7, 0xb, 0x27, 0x3, 0x27, 0x3, 0x27, 0x3, 0x27, 0x3, 0x27, 0x5, 0x27, 
    0x2ad, 0xa, 0x27, 0x3, 0x28, 0x3, 0x28, 0x5, 0x28, 0x2b1, 0xa, 0x28, 
    0x3, 0x28, 0x3, 0x28, 0x5, 0x28, 0x2b5, 0xa, 0x28, 0x5, 0x28, 0x2b7, 
    0xa, 0x28, 0x3, 0x28, 0x3, 0x28, 0x5, 0x28, 0x2bb, 0xa, 0x28, 0x5, 0x28, 
    0x2bd, 0xa, 0x28, 0x3, 0x28, 0x3, 0x28, 0x5, 0x28, 0x2c1, 0xa, 0x28, 
    0x5, 0x28, 0x2c3, 0xa, 0x28, 0x3, 0x28, 0x3, 0x28, 0x3, 0x29, 0x3, 0x29, 
    0x5, 0x29, 0x2c9, 0xa, 0x29, 0x3, 0x29, 0x3, 0x29, 0x3, 0x2a, 0x3, 0x2a, 
    0x5, 0x2a, 0x2cf, 0xa, 0x2a, 0x3, 0x2a, 0x3, 0x2a, 0x5, 0x2a, 0x2d3, 
    0xa, 0x2a, 0x3, 0x2a, 0x5, 0x2a, 0x2d6, 0xa, 0x2a, 0x3, 0x2a, 0x5, 0x2a, 
    0x2d9, 0xa, 0x2a, 0x3, 0x2a, 0x3, 0x2a, 0x5, 0x2a, 0x2dd, 0xa, 0x2a, 
    0x3, 0x2a, 0x3, 0x2a, 0x3, 0x2a, 0x3, 0x2a, 0x5, 0x2a, 0x2e3, 0xa, 0x2a, 
    0x3, 0x2a, 0x3, 0x2a, 0x5, 0x2a, 0x2e7, 0xa, 0x2a, 0x3, 0x2a, 0x5, 0x2a, 
    0x2ea, 0xa, 0x2a, 0x3, 0x2a, 0x5, 0x2a, 0x2ed, 0xa, 0x2a, 0x3, 0x2a, 
    0x3, 0x2a, 0x3, 0x2a, 0x3, 0x2a, 0x5, 0x2a, 0x2f3, 0xa, 0x2a, 0x3, 0x2a, 
    0x5, 0x2a, 0x2f6, 0xa, 0x2a, 0x3, 0x2a, 0x5, 0x2a, 0x2f9, 0xa, 0x2a, 
    0x3, 0x2a, 0x3, 0x2a, 0x5, 0x2a, 0x2fd, 0xa, 0x2a, 0x3, 0x2a, 0x3, 0x2a, 
    0x3, 0x2a, 0x3, 0x2a, 0x5, 0x2a, 0x303, 0xa, 0x2a, 0x3, 0x2a, 0x5, 0x2a, 
    0x306, 0xa, 0x2a, 0x3, 0x2a, 0x5, 0x2a, 0x309, 0xa, 0x2a, 0x3, 0x2a, 
    0x3, 0x2a, 0x5, 0x2a, 0x30d, 0xa, 0x2a, 0x3, 0x2b, 0x3, 0x2b, 0x5, 0x2b, 
    0x311, 0xa, 0x2b, 0x3, 0x2b, 0x3, 0x2b, 0x5, 0x2b, 0x315, 0xa, 0x2b, 
    0x5, 0x2b, 0x317, 0xa, 0x2b, 0x3, 0x2b, 0x3, 0x2b, 0x5, 0x2b, 0x31b, 
    0xa, 0x2b, 0x5, 0x2b, 0x31d, 0xa, 0x2b, 0x3, 0x2b, 0x5, 0x2b, 0x320, 
    0xa, 0x2b, 0x3, 0x2b, 0x3, 0x2b, 0x5, 0x2b, 0x324, 0xa, 0x2b, 0x5, 0x2b, 
    0x326, 0xa, 0x2b, 0x3, 0x2b, 0x3, 0x2b, 0x3, 0x2c, 0x3, 0x2c, 0x5, 0x2c, 
    0x32c, 0xa, 0x2c, 0x3, 0x2d, 0x3, 0x2d, 0x5, 0x2d, 0x330, 0xa, 0x2d, 
    0x3, 0x2d, 0x3, 0x2d, 0x5, 0x2d, 0x334, 0xa, 0x2d, 0x3, 0x2d, 0x3, 0x2d, 
    0x5, 0x2d, 0x338, 0xa, 0x2d, 0x3, 0x2d, 0x5, 0x2d, 0x33b, 0xa, 0x2d, 
    0x3, 0x2d, 0x7, 0x2d, 0x33e, 0xa, 0x2d, 0xc, 0x2d, 0xe, 0x2d, 0x341, 
    0xb, 0x2d, 0x3, 0x2e, 0x3, 0x2e, 0x5, 0x2e, 0x345, 0xa, 0x2e, 0x3, 0x2e, 
    0x7, 0x2e, 0x348, 0xa, 0x2e, 0xc, 0x2e, 0xe, 0x2e, 0x34b, 0xb, 0x2e, 
    0x3, 0x2f, 0x3, 0x2f, 0x5, 0x2f, 0x34f, 0xa, 0x2f, 0x3, 0x2f, 0x3, 0x2f, 
    0x3, 0x30, 0x3, 0x30, 0x5, 0x30, 0x355, 0xa, 0x30, 0x3, 0x30, 0x3, 0x30, 
    0x5, 0x30, 0x359, 0xa, 0x30, 0x5, 0x30, 0x35b, 0xa, 0x30, 0x3, 0x30, 
    0x3, 0x30, 0x5, 0x30, 0x35f, 0xa, 0x30, 0x3, 0x30, 0x3, 0x30, 0x5, 0x30, 
    0x363, 0xa, 0x30, 0x5, 0x30, 0x365, 0xa, 0x30, 0x5, 0x30, 0x367, 0xa, 
    0x30, 0x3, 0x31, 0x3, 0x31, 0x3, 0x32, 0x3, 0x32, 0x3, 0x33, 0x3, 0x33, 
    0x3, 0x34, 0x3, 0x34, 0x3, 0x34, 0x3, 0x34, 0x3, 0x34, 0x7, 0x34, 0x374, 
    0xa, 0x34, 0xc, 0x34, 0xe, 0x34, 0x377, 0xb, 0x34, 0x3, 0x35, 0x3, 0x35, 
    0x3, 0x35, 0x3, 0x35, 0x3, 0x35, 0x7, 0x35, 0x37e, 0xa, 0x35, 0xc, 0x35, 
    0xe, 0x35, 0x381, 0xb, 0x35, 0x3, 0x36, 0x3, 0x36, 0x3, 0x36, 0x3, 0x36, 
    0x3, 0x36, 0x7, 0x36, 0x388, 0xa, 0x36, 0xc, 0x36, 0xe, 0x36, 0x38b, 
    0xb, 0x36, 0x3, 0x37, 0x3, 0x37, 0x5, 0x37, 0x38f, 0xa, 0x37, 0x7, 0x37, 
    0x391, 0xa, 0x37, 0xc, 0x37, 0xe, 0x37, 0x394, 0xb, 0x37, 0x3, 0x37, 
    0x3, 0x37, 0x3, 0x38, 0x3, 0x38, 0x5, 0x38, 0x39a, 0xa, 0x38, 0x3, 0x38, 
    0x7, 0x38, 0x39d, 0xa, 0x38, 0xc, 0x38, 0xe, 0x38, 0x3a0, 0xb, 0x38, 
    0x3, 0x39, 0x3, 0x39, 0x5, 0x39, 0x3a4, 0xa, 0x39, 0x3, 0x39, 0x3, 0x39, 
    0x5, 0x39, 0x3a8, 0xa, 0x39, 0x3, 0x39, 0x3, 0x39, 0x5, 0x39, 0x3ac, 
    0xa, 0x39, 0x3, 0x39, 0x3, 0x39, 0x5, 0x39, 0x3b0, 0xa, 0x39, 0x3, 0x39, 
    0x7, 0x39, 0x3b3, 0xa, 0x39, 0xc, 0x39, 0xe, 0x39, 0x3b6, 0xb, 0x39, 
    0x3, 0x3a, 0x3, 0x3a, 0x5, 0x3a, 0x3ba, 0xa, 0x3a, 0x3, 0x3a, 0x3, 0x3a, 
    0x5, 0x3a, 0x3be, 0xa, 0x3a, 0x3, 0x3a, 0x3, 0x3a, 0x5, 0x3a, 0x3c2, 
    0xa, 0x3a, 0x3, 0x3a, 0x3, 0x3a, 0x5, 0x3a, 0x3c6, 0xa, 0x3a, 0x3, 0x3a, 
    0x3, 0x3a, 0x5, 0x3a, 0x3ca, 0xa, 0x3a, 0x3, 0x3a, 0x3, 0x3a, 0x5, 0x3a, 
    0x3ce, 0xa, 0x3a, 0x3, 0x3a, 0x7, 0x3a, 0x3d1, 0xa, 0x3a, 0xc, 0x3a, 
    0xe, 0x3a, 0x3d4, 0xb, 0x3a, 0x3, 0x3b, 0x3, 0x3b, 0x5, 0x3b, 0x3d8, 
    0xa, 0x3b, 0x3, 0x3b, 0x3, 0x3b, 0x5, 0x3b, 0x3dc, 0xa, 0x3b, 0x3, 0x3b, 
    0x7, 0x3b, 0x3df, 0xa, 0x3b, 0xc, 0x3b, 0xe, 0x3b, 0x3e2, 0xb, 0x3b, 
    0x3, 0x3c, 0x3, 0x3c, 0x5, 0x3c, 0x3e6, 0xa, 0x3c, 0x7, 0x3c, 0x3e8, 
    0xa, 0x3c, 0xc, 0x3c, 0xe, 0x3c, 0x3eb, 0xb, 0x3c, 0x3, 0x3c, 0x3, 0x3c, 
    0x3, 0x3d, 0x3, 0x3d, 0x3, 0x3d, 0x3, 0x3d, 0x7, 0x3d, 0x3f3, 0xa, 0x3d, 
    0xc, 0x3d, 0xe, 0x3d, 0x3f6, 0xb, 0x3d, 0x3, 0x3e, 0x3, 0x3e, 0x3, 0x3e, 
    0x5, 0x3e, 0x3fb, 0xa, 0x3e, 0x3, 0x3e, 0x3, 0x3e, 0x5, 0x3e, 0x3ff, 
    0xa, 0x3e, 0x3, 0x3e, 0x3, 0x3e, 0x3, 0x3e, 0x3, 0x3e, 0x3, 0x3e, 0x5, 
    0x3e, 0x406, 0xa, 0x3e, 0x3, 0x3e, 0x3, 0x3e, 0x5, 0x3e, 0x40a, 0xa, 
    0x3e, 0x3, 0x3e, 0x3, 0x3e, 0x5, 0x3e, 0x40e, 0xa, 0x3e, 0x3, 0x3e, 
    0x5, 0x3e, 0x411, 0xa, 0x3e, 0x3, 0x3f, 0x3, 0x3f, 0x3, 0x3f, 0x3, 0x3f, 
    0x3, 0x3f, 0x3, 0x3f, 0x3, 0x3f, 0x3, 0x3f, 0x3, 0x3f, 0x3, 0x3f, 0x5, 
    0x3f, 0x41d, 0xa, 0x3f, 0x3, 0x3f, 0x5, 0x3f, 0x420, 0xa, 0x3f, 0x3, 
    0x3f, 0x3, 0x3f, 0x3, 0x40, 0x3, 0x40, 0x3, 0x40, 0x3, 0x40, 0x3, 0x40, 
    0x3, 0x40, 0x3, 0x40, 0x3, 0x40, 0x3, 0x40, 0x3, 0x40, 0x5, 0x40, 0x42e, 
    0xa, 0x40, 0x3, 0x41, 0x3, 0x41, 0x5, 0x41, 0x432, 0xa, 0x41, 0x3, 0x41, 
    0x7, 0x41, 0x435, 0xa, 0x41, 0xc, 0x41, 0xe, 0x41, 0x438, 0xb, 0x41, 
    0x3, 0x41, 0x5, 0x41, 0x43b, 0xa, 0x41, 0x3, 0x41, 0x5, 0x41, 0x43e, 
    0xa, 0x41, 0x3, 0x42, 0x3, 0x42, 0x3, 0x42, 0x3, 0x42, 0x3, 0x42, 0x5, 
    0x42, 0x445, 0xa, 0x42, 0x3, 0x42, 0x3, 0x42, 0x5, 0x42, 0x449, 0xa, 
    0x42, 0x3, 0x42, 0x3, 0x42, 0x5, 0x42, 0x44d, 0xa, 0x42, 0x3, 0x42, 
    0x3, 0x42, 0x3, 0x42, 0x3, 0x42, 0x3, 0x42, 0x5, 0x42, 0x454, 0xa, 0x42, 
    0x3, 0x42, 0x3, 0x42, 0x5, 0x42, 0x458, 0xa, 0x42, 0x3, 0x42, 0x3, 0x42, 
    0x5, 0x42, 0x45c, 0xa, 0x42, 0x3, 0x42, 0x3, 0x42, 0x3, 0x42, 0x3, 0x42, 
    0x5, 0x42, 0x462, 0xa, 0x42, 0x3, 0x42, 0x3, 0x42, 0x5, 0x42, 0x466, 
    0xa, 0x42, 0x3, 0x42, 0x3, 0x42, 0x5, 0x42, 0x46a, 0xa, 0x42, 0x3, 0x42, 
    0x3, 0x42, 0x3, 0x42, 0x3, 0x42, 0x5, 0x42, 0x470, 0xa, 0x42, 0x3, 0x42, 
    0x3, 0x42, 0x5, 0x42, 0x474, 0xa, 0x42, 0x3, 0x42, 0x3, 0x42, 0x5, 0x42, 
    0x478, 0xa, 0x42, 0x3, 0x42, 0x3, 0x42, 0x3, 0x42, 0x3, 0x42, 0x5, 0x42, 
    0x47e, 0xa, 0x42, 0x3, 0x42, 0x3, 0x42, 0x5, 0x42, 0x482, 0xa, 0x42, 
    0x3, 0x42, 0x3, 0x42, 0x5, 0x42, 0x486, 0xa, 0x42, 0x3, 0x42, 0x3, 0x42, 
    0x3, 0x42, 0x3, 0x42, 0x3, 0x42, 0x3, 0x42, 0x5, 0x42, 0x48e, 0xa, 0x42, 
    0x3, 0x43, 0x3, 0x43, 0x3, 0x43, 0x3, 0x43, 0x3, 0x43, 0x3, 0x43, 0x5, 
    0x43, 0x496, 0xa, 0x43, 0x3, 0x44, 0x3, 0x44, 0x3, 0x45, 0x3, 0x45, 
    0x5, 0x45, 0x49c, 0xa, 0x45, 0x3, 0x45, 0x3, 0x45, 0x5, 0x45, 0x4a0, 
    0xa, 0x45, 0x3, 0x45, 0x3, 0x45, 0x5, 0x45, 0x4a4, 0xa, 0x45, 0x3, 0x45, 
    0x3, 0x45, 0x5, 0x45, 0x4a8, 0xa, 0x45, 0x7, 0x45, 0x4aa, 0xa, 0x45, 
    0xc, 0x45, 0xe, 0x45, 0x4ad, 0xb, 0x45, 0x5, 0x45, 0x4af, 0xa, 0x45, 
    0x3, 0x45, 0x3, 0x45, 0x3, 0x46, 0x3, 0x46, 0x5, 0x46, 0x4b5, 0xa, 0x46, 
    0x3, 0x46, 0x3, 0x46, 0x3, 0x46, 0x5, 0x46, 0x4ba, 0xa, 0x46, 0x3, 0x46, 
    0x3, 0x46, 0x3, 0x46, 0x5, 0x46, 0x4bf, 0xa, 0x46, 0x3, 0x46, 0x3, 0x46, 
    0x3, 0x46, 0x5, 0x46, 0x4c4, 0xa, 0x46, 0x3, 0x46, 0x3, 0x46, 0x3, 0x46, 
    0x5, 0x46, 0x4c9, 0xa, 0x46, 0x3, 0x46, 0x3, 0x46, 0x3, 0x46, 0x5, 0x46, 
    0x4ce, 0xa, 0x46, 0x3, 0x46, 0x5, 0x46, 0x4d1, 0xa, 0x46, 0x3, 0x47, 
    0x3, 0x47, 0x5, 0x47, 0x4d5, 0xa, 0x47, 0x3, 0x47, 0x3, 0x47, 0x5, 0x47, 
    0x4d9, 0xa, 0x47, 0x3, 0x47, 0x3, 0x47, 0x3, 0x48, 0x3, 0x48, 0x5, 0x48, 
    0x4df, 0xa, 0x48, 0x3, 0x48, 0x6, 0x48, 0x4e2, 0xa, 0x48, 0xd, 0x48, 
    0xe, 0x48, 0x4e3, 0x3, 0x49, 0x3, 0x49, 0x5, 0x49, 0x4e8, 0xa, 0x49, 
    0x3, 0x49, 0x5, 0x49, 0x4eb, 0xa, 0x49, 0x3, 0x4a, 0x3, 0x4a, 0x3, 0x4a, 
    0x3, 0x4a, 0x3, 0x4a, 0x3, 0x4a, 0x3, 0x4b, 0x3, 0x4b, 0x5, 0x4b, 0x4f5, 
    0xa, 0x4b, 0x3, 0x4b, 0x3, 0x4b, 0x5, 0x4b, 0x4f9, 0xa, 0x4b, 0x3, 0x4b, 
    0x3, 0x4b, 0x5, 0x4b, 0x4fd, 0xa, 0x4b, 0x5, 0x4b, 0x4ff, 0xa, 0x4b, 
    0x3, 0x4b, 0x3, 0x4b, 0x5, 0x4b, 0x503, 0xa, 0x4b, 0x3, 0x4b, 0x3, 0x4b, 
    0x5, 0x4b, 0x507, 0xa, 0x4b, 0x3, 0x4b, 0x3, 0x4b, 0x5, 0x4b, 0x50b, 
    0xa, 0x4b, 0x7, 0x4b, 0x50d, 0xa, 0x4b, 0xc, 0x4b, 0xe, 0x4b, 0x510, 
    0xb, 0x4b, 0x5, 0x4b, 0x512, 0xa, 0x4b, 0x3, 0x4b, 0x3, 0x4b, 0x3, 0x4c, 
    0x3, 0x4c, 0x3, 0x4c, 0x3, 0x4c, 0x5, 0x4c, 0x51a, 0xa, 0x4c, 0x3, 0x4d, 
    0x3, 0x4d, 0x5, 0x4d, 0x51e, 0xa, 0x4d, 0x3, 0x4d, 0x3, 0x4d, 0x5, 0x4d, 
    0x522, 0xa, 0x4d, 0x3, 0x4d, 0x3, 0x4d, 0x5, 0x4d, 0x526, 0xa, 0x4d, 
    0x3, 0x4d, 0x3, 0x4d, 0x5, 0x4d, 0x52a, 0xa, 0x4d, 0x3, 0x4d, 0x3, 0x4d, 
    0x5, 0x4d, 0x52e, 0xa, 0x4d, 0x7, 0x4d, 0x530, 0xa, 0x4d, 0xc, 0x4d, 
    0xe, 0x4d, 0x533, 0xb, 0x4d, 0x5, 0x4d, 0x535, 0xa, 0x4d, 0x3, 0x4d, 
    0x3, 0x4d, 0x3, 0x4e, 0x3, 0x4e, 0x3, 0x4f, 0x3, 0x4f, 0x3, 0x50, 0x3, 
    0x50, 0x3, 0x50, 0x3, 0x51, 0x3, 0x51, 0x3, 0x51, 0x7, 0x51, 0x543, 
    0xa, 0x51, 0xc, 0x51, 0xe, 0x51, 0x546, 0xb, 0x51, 0x3, 0x52, 0x3, 0x52, 
    0x5, 0x52, 0x54a, 0xa, 0x52, 0x3, 0x52, 0x3, 0x52, 0x5, 0x52, 0x54e, 
    0xa, 0x52, 0x3, 0x52, 0x3, 0x52, 0x5, 0x52, 0x552, 0xa, 0x52, 0x3, 0x52, 
    0x5, 0x52, 0x555, 0xa, 0x52, 0x3, 0x52, 0x5, 0x52, 0x558, 0xa, 0x52, 
    0x3, 0x52, 0x3, 0x52, 0x3, 0x53, 0x3, 0x53, 0x5, 0x53, 0x55e, 0xa, 0x53, 
    0x3, 0x53, 0x3, 0x53, 0x5, 0x53, 0x562, 0xa, 0x53, 0x3, 0x53, 0x3, 0x53, 
    0x5, 0x53, 0x566, 0xa, 0x53, 0x5, 0x53, 0x568, 0xa, 0x53, 0x3, 0x53, 
    0x3, 0x53, 0x5, 0x53, 0x56c, 0xa, 0x53, 0x3, 0x53, 0x3, 0x53, 0x5, 0x53, 
    0x570, 0xa, 0x53, 0x3, 0x53, 0x3, 0x53, 0x5, 0x53, 0x574, 0xa, 0x53, 
    0x5, 0x53, 0x576, 0xa, 0x53, 0x3, 0x53, 0x3, 0x53, 0x5, 0x53, 0x57a, 
    0xa, 0x53, 0x3, 0x53, 0x3, 0x53, 0x5, 0x53, 0x57e, 0xa, 0x53, 0x3, 0x53, 
    0x3, 0x53, 0x3, 0x54, 0x3, 0x54, 0x5, 0x54, 0x584, 0xa, 0x54, 0x3, 0x54, 
    0x3, 0x54, 0x3, 0x55, 0x3, 0x55, 0x5, 0x55, 0x58a, 0xa, 0x55, 0x3, 0x55, 
    0x6, 0x55, 0x58d, 0xa, 0x55, 0xd, 0x55, 0xe, 0x55, 0x58e, 0x3, 0x55, 
    0x3, 0x55, 0x5, 0x55, 0x593, 0xa, 0x55, 0x3, 0x55, 0x3, 0x55, 0x5, 0x55, 
    0x597, 0xa, 0x55, 0x3, 0x55, 0x6, 0x55, 0x59a, 0xa, 0x55, 0xd, 0x55, 
    0xe, 0x55, 0x59b, 0x5, 0x55, 0x59e, 0xa, 0x55, 0x3, 0x55, 0x5, 0x55, 
    0x5a1, 0xa, 0x55, 0x3, 0x55, 0x3, 0x55, 0x5, 0x55, 0x5a5, 0xa, 0x55, 
    0x3, 0x55, 0x5, 0x55, 0x5a8, 0xa, 0x55, 0x3, 0x55, 0x5, 0x55, 0x5ab, 
    0xa, 0x55, 0x3, 0x55, 0x3, 0x55, 0x3, 0x56, 0x3, 0x56, 0x5, 0x56, 0x5b1, 
    0xa, 0x56, 0x3, 0x56, 0x3, 0x56, 0x5, 0x56, 0x5b5, 0xa, 0x56, 0x3, 0x56, 
    0x3, 0x56, 0x5, 0x56, 0x5b9, 0xa, 0x56, 0x3, 0x56, 0x3, 0x56, 0x3, 0x57, 
    0x3, 0x57, 0x3, 0x58, 0x3, 0x58, 0x5, 0x58, 0x5c1, 0xa, 0x58, 0x3, 0x59, 
    0x3, 0x59, 0x5, 0x59, 0x5c5, 0xa, 0x59, 0x3, 0x59, 0x3, 0x59, 0x5, 0x59, 
    0x5c9, 0xa, 0x59, 0x3, 0x59, 0x3, 0x59, 0x5, 0x59, 0x5cd, 0xa, 0x59, 
    0x3, 0x59, 0x3, 0x59, 0x5, 0x59, 0x5d1, 0xa, 0x59, 0x3, 0x59, 0x3, 0x59, 
    0x5, 0x59, 0x5d5, 0xa, 0x59, 0x3, 0x59, 0x3, 0x59, 0x5, 0x59, 0x5d9, 
    0xa, 0x59, 0x3, 0x59, 0x3, 0x59, 0x5, 0x59, 0x5dd, 0xa, 0x59, 0x3, 0x59, 
    0x3, 0x59, 0x5, 0x59, 0x5e1, 0xa, 0x59, 0x7, 0x59, 0x5e3, 0xa, 0x59, 
    0xc, 0x59, 0xe, 0x59, 0x5e6, 0xb, 0x59, 0x5, 0x59, 0x5e8, 0xa, 0x59, 
    0x3, 0x59, 0x3, 0x59, 0x3, 0x5a, 0x3, 0x5a, 0x3, 0x5a, 0x5, 0x5a, 0x5ef, 
    0xa, 0x5a, 0x3, 0x5b, 0x3, 0x5b, 0x5, 0x5b, 0x5f3, 0xa, 0x5b, 0x3, 0x5b, 
    0x6, 0x5b, 0x5f6, 0xa, 0x5b, 0xd, 0x5b, 0xe, 0x5b, 0x5f7, 0x3, 0x5c, 
    0x3, 0x5c, 0x3, 0x5d, 0x3, 0x5d, 0x3, 0x5e, 0x3, 0x5e, 0x3, 0x5f, 0x3, 
    0x5f, 0x5, 0x5f, 0x602, 0xa, 0x5f, 0x3, 0x60, 0x3, 0x60, 0x3, 0x61, 
    0x3, 0x61, 0x3, 0x62, 0x3, 0x62, 0x3, 0x63, 0x3, 0x63, 0x3, 0x64, 0x3, 
    0x64, 0x3, 0x64, 0x2, 0x2, 0x65, 0x2, 0x4, 0x6, 0x8, 0xa, 0xc, 0xe, 
    0x10, 0x12, 0x14, 0x16, 0x18, 0x1a, 0x1c, 0x1e, 0x20, 0x22, 0x24, 0x26, 
    0x28, 0x2a, 0x2c, 0x2e, 0x30, 0x32, 0x34, 0x36, 0x38, 0x3a, 0x3c, 0x3e, 
    0x40, 0x42, 0x44, 0x46, 0x48, 0x4a, 0x4c, 0x4e, 0x50, 0x52, 0x54, 0x56, 
    0x58, 0x5a, 0x5c, 0x5e, 0x60, 0x62, 0x64, 0x66, 0x68, 0x6a, 0x6c, 0x6e, 
    0x70, 0x72, 0x74, 0x76, 0x78, 0x7a, 0x7c, 0x7e, 0x80, 0x82, 0x84, 0x86, 
    0x88, 0x8a, 0x8c, 0x8e, 0x90, 0x92, 0x94, 0x96, 0x98, 0x9a, 0x9c, 0x9e, 
    0xa0, 0xa2, 0xa4, 0xa6, 0xa8, 0xaa, 0xac, 0xae, 0xb0, 0xb2, 0xb4, 0xb6, 
    0xb8, 0xba, 0xbc, 0xbe, 0xc0, 0xc2, 0xc4, 0xc6, 0x2, 0xc, 0x3, 0x2, 
    0x46, 0x49, 0x3, 0x2, 0xf, 0x10, 0x3, 0x2, 0x59, 0x5a, 0x3, 0x2, 0x63, 
    0x65, 0x3, 0x2, 0x6d, 0x6e, 0x6, 0x2, 0x30, 0x3c, 0x3f, 0x54, 0x59, 
    0x60, 0x6f, 0x78, 0x6, 0x2, 0x55, 0x58, 0x66, 0x66, 0x79, 0x7b, 0x7e, 
    0x7e, 0x4, 0x2, 0x15, 0x15, 0x1d, 0x20, 0x4, 0x2, 0x16, 0x16, 0x21, 
    0x24, 0x4, 0x2, 0x10, 0x10, 0x25, 0x2f, 0x2, 0x6e7, 0x2, 0xc9, 0x3, 
    0x2, 0x2, 0x2, 0x4, 0xd7, 0x3, 0x2, 0x2, 0x2, 0x6, 0xdb, 0x3, 0x2, 0x2, 
    0x2, 0x8, 0xdd, 0x3, 0x2, 0x2, 0x2, 0xa, 0xf3, 0x3, 0x2, 0x2, 0x2, 0xc, 
    0xf7, 0x3, 0x2, 0x2, 0x2, 0xe, 0x11c, 0x3, 0x2, 0x2, 0x2, 0x10, 0x134, 
    0x3, 0x2, 0x2, 0x2, 0x12, 0x13f, 0x3, 0x2, 0x2, 0x2, 0x14, 0x144, 0x3, 
    0x2, 0x2, 0x2, 0x16, 0x148, 0x3, 0x2, 0x2, 0x2, 0x18, 0x155, 0x3, 0x2, 
    0x2, 0x2, 0x1a, 0x15f, 0x3, 0x2, 0x2, 0x2, 0x1c, 0x175, 0x3, 0x2, 0x2, 
    0x2, 0x1e, 0x177, 0x3, 0x2, 0x2, 0x2, 0x20, 0x17d, 0x3, 0x2, 0x2, 0x2, 
    0x22, 0x1ad, 0x3, 0x2, 0x2, 0x2, 0x24, 0x1b1, 0x3, 0x2, 0x2, 0x2, 0x26, 
    0x1c5, 0x3, 0x2, 0x2, 0x2, 0x28, 0x1d9, 0x3, 0x2, 0x2, 0x2, 0x2a, 0x1db, 
    0x3, 0x2, 0x2, 0x2, 0x2c, 0x1e6, 0x3, 0x2, 0x2, 0x2, 0x2e, 0x201, 0x3, 
    0x2, 0x2, 0x2, 0x30, 0x20e, 0x3, 0x2, 0x2, 0x2, 0x32, 0x212, 0x3, 0x2, 
    0x2, 0x2, 0x34, 0x221, 0x3, 0x2, 0x2, 0x2, 0x36, 0x22b, 0x3, 0x2, 0x2, 
    0x2, 0x38, 0x254, 0x3, 0x2, 0x2, 0x2, 0x3a, 0x25d, 0x3, 0x2, 0x2, 0x2, 
    0x3c, 0x25f, 0x3, 0x2, 0x2, 0x2, 0x3e, 0x26e, 0x3, 0x2, 0x2, 0x2, 0x40, 
    0x272, 0x3, 0x2, 0x2, 0x2, 0x42, 0x276, 0x3, 0x2, 0x2, 0x2, 0x44, 0x27d, 
    0x3, 0x2, 0x2, 0x2, 0x46, 0x281, 0x3, 0x2, 0x2, 0x2, 0x48, 0x29a, 0x3, 
    0x2, 0x2, 0x2, 0x4a, 0x29c, 0x3, 0x2, 0x2, 0x2, 0x4c, 0x2ac, 0x3, 0x2, 
    0x2, 0x2, 0x4e, 0x2ae, 0x3, 0x2, 0x2, 0x2, 0x50, 0x2c6, 0x3, 0x2, 0x2, 
    0x2, 0x52, 0x30c, 0x3, 0x2, 0x2, 0x2, 0x54, 0x30e, 0x3, 0x2, 0x2, 0x2, 
    0x56, 0x32b, 0x3, 0x2, 0x2, 0x2, 0x58, 0x32d, 0x3, 0x2, 0x2, 0x2, 0x5a, 
    0x342, 0x3, 0x2, 0x2, 0x2, 0x5c, 0x34c, 0x3, 0x2, 0x2, 0x2, 0x5e, 0x352, 
    0x3, 0x2, 0x2, 0x2, 0x60, 0x368, 0x3, 0x2, 0x2, 0x2, 0x62, 0x36a, 0x3, 
    0x2, 0x2, 0x2, 0x64, 0x36c, 0x3, 0x2, 0x2, 0x2, 0x66, 0x36e, 0x3, 0x2, 
    0x2, 0x2, 0x68, 0x378, 0x3, 0x2, 0x2, 0x2, 0x6a, 0x382, 0x3, 0x2, 0x2, 
    0x2, 0x6c, 0x392, 0x3, 0x2, 0x2, 0x2, 0x6e, 0x397, 0x3, 0x2, 0x2, 0x2, 
    0x70, 0x3a1, 0x3, 0x2, 0x2, 0x2, 0x72, 0x3b7, 0x3, 0x2, 0x2, 0x2, 0x74, 
    0x3d5, 0x3, 0x2, 0x2, 0x2, 0x76, 0x3e9, 0x3, 0x2, 0x2, 0x2, 0x78, 0x3ee, 
    0x3, 0x2, 0x2, 0x2, 0x7a, 0x410, 0x3, 0x2, 0x2, 0x2, 0x7c, 0x41c, 0x3, 
    0x2, 0x2, 0x2, 0x7e, 0x42d, 0x3, 0x2, 0x2, 0x2, 0x80, 0x42f, 0x3, 0x2, 
    0x2, 0x2, 0x82, 0x48d, 0x3, 0x2, 0x2, 0x2, 0x84, 0x495, 0x3, 0x2, 0x2, 
    0x2, 0x86, 0x497, 0x3, 0x2, 0x2, 0x2, 0x88, 0x499, 0x3, 0x2, 0x2, 0x2, 
    0x8a, 0x4d0, 0x3, 0x2, 0x2, 0x2, 0x8c, 0x4d2, 0x3, 0x2, 0x2, 0x2, 0x8e, 
    0x4dc, 0x3, 0x2, 0x2, 0x2, 0x90, 0x4e5, 0x3, 0x2, 0x2, 0x2, 0x92, 0x4ec, 
    0x3, 0x2, 0x2, 0x2, 0x94, 0x4f2, 0x3, 0x2, 0x2, 0x2, 0x96, 0x519, 0x3, 
    0x2, 0x2, 0x2, 0x98, 0x51b, 0x3, 0x2, 0x2, 0x2, 0x9a, 0x538, 0x3, 0x2, 
    0x2, 0x2, 0x9c, 0x53a, 0x3, 0x2, 0x2, 0x2, 0x9e, 0x53c, 0x3, 0x2, 0x2, 
    0x2, 0xa0, 0x544, 0x3, 0x2, 0x2, 0x2, 0xa2, 0x547, 0x3, 0x2, 0x2, 0x2, 
    0xa4, 0x55b, 0x3, 0x2, 0x2, 0x2, 0xa6, 0x581, 0x3, 0x2, 0x2, 0x2, 0xa8, 
    0x59d, 0x3, 0x2, 0x2, 0x2, 0xaa, 0x5ae, 0x3, 0x2, 0x2, 0x2, 0xac, 0x5bc, 
    0x3, 0x2, 0x2, 0x2, 0xae, 0x5c0, 0x3, 0x2, 0x2, 0x2, 0xb0, 0x5c2, 0x3, 
    0x2, 0x2, 0x2, 0xb2, 0x5eb, 0x3, 0x2, 0x2, 0x2, 0xb4, 0x5f0, 0x3, 0x2, 
    0x2, 0x2, 0xb6, 0x5f9, 0x3, 0x2, 0x2, 0x2, 0xb8, 0x5fb, 0x3, 0x2, 0x2, 
    0x2, 0xba, 0x5fd, 0x3, 0x2, 0x2, 0x2, 0xbc, 0x601, 0x3, 0x2, 0x2, 0x2, 
    0xbe, 0x603, 0x3, 0x2, 0x2, 0x2, 0xc0, 0x605, 0x3, 0x2, 0x2, 0x2, 0xc2, 
    0x607, 0x3, 0x2, 0x2, 0x2, 0xc4, 0x609, 0x3, 0x2, 0x2, 0x2, 0xc6, 0x60b, 
    0x3, 0x2, 0x2, 0x2, 0xc8, 0xca, 0x7, 0x7f, 0x2, 0x2, 0xc9, 0xc8, 0x3, 
    0x2, 0x2, 0x2, 0xc9, 0xca, 0x3, 0x2, 0x2, 0x2, 0xca, 0xcb, 0x3, 0x2, 
    0x2, 0x2, 0xcb, 0xd0, 0x5, 0x4, 0x3, 0x2, 0xcc, 0xce, 0x7, 0x7f, 0x2, 
    0x2, 0xcd, 0xcc, 0x3, 0x2, 0x2, 0x2, 0xcd, 0xce, 0x3, 0x2, 0x2, 0x2, 
    0xce, 0xcf, 0x3, 0x2, 0x2, 0x2, 0xcf, 0xd1, 0x7, 0x3, 0x2, 0x2, 0xd0, 
    0xcd, 0x3, 0x2, 0x2, 0x2, 0xd0, 0xd1, 0x3, 0x2, 0x2, 0x2, 0xd1, 0xd3, 
    0x3, 0x2, 0x2, 0x2, 0xd2, 0xd4, 0x7, 0x7f, 0x2, 0x2, 0xd3, 0xd2, 0x3, 
    0x2, 0x2, 0x2, 0xd3, 0xd4, 0x3, 0x2, 0x2, 0x2, 0xd4, 0xd5, 0x3, 0x2, 
    0x2, 0x2, 0xd5, 0xd6, 0x7, 0x2, 0x2, 0x3, 0xd6, 0x3, 0x3, 0x2, 0x2, 
    0x2, 0xd7, 0xd8, 0x5, 0x6, 0x4, 0x2, 0xd8, 0x5, 0x3, 0x2, 0x2, 0x2, 
    0xd9, 0xdc, 0x5, 0x8, 0x5, 0x2, 0xda, 0xdc, 0x5, 0x2c, 0x17, 0x2, 0xdb, 
    0xd9, 0x3, 0x2, 0x2, 0x2, 0xdb, 0xda, 0x3, 0x2, 0x2, 0x2, 0xdc, 0x7, 
    0x3, 0x2, 0x2, 0x2, 0xdd, 0xe4, 0x5, 0xc, 0x7, 0x2, 0xde, 0xe0, 0x7, 
    0x7f, 0x2, 0x2, 0xdf, 0xde, 0x3, 0x2, 0x2, 0x2, 0xdf, 0xe0, 0x3, 0x2, 
    0x2, 0x2, 0xe0, 0xe1, 0x3, 0x2, 0x2, 0x2, 0xe1, 0xe3, 0x5, 0xa, 0x6, 
    0x2, 0xe2, 0xdf, 0x3, 0x2, 0x2, 0x2, 0xe3, 0xe6, 0x3, 0x2, 0x2, 0x2, 
    0xe4, 0xe2, 0x3, 0x2, 0x2, 0x2, 0xe4, 0xe5, 0x3, 0x2, 0x2, 0x2, 0xe5, 
    0x9, 0x3, 0x2, 0x2, 0x2, 0xe6, 0xe4, 0x3, 0x2, 0x2, 0x2, 0xe7, 0xe8, 
    0x7, 0x30, 0x2, 0x2, 0xe8, 0xe9, 0x7, 0x7f, 0x2, 0x2, 0xe9, 0xeb, 0x7, 
    0x31, 0x2, 0x2, 0xea, 0xec, 0x7, 0x7f, 0x2, 0x2, 0xeb, 0xea, 0x3, 0x2, 
    0x2, 0x2, 0xeb, 0xec, 0x3, 0x2, 0x2, 0x2, 0xec, 0xed, 0x3, 0x2, 0x2, 
    0x2, 0xed, 0xf4, 0x5, 0xc, 0x7, 0x2, 0xee, 0xf0, 0x7, 0x30, 0x2, 0x2, 
    0xef, 0xf1, 0x7, 0x7f, 0x2, 0x2, 0xf0, 0xef, 0x3, 0x2, 0x2, 0x2, 0xf0, 
    0xf1, 0x3, 0x2, 0x2, 0x2, 0xf1, 0xf2, 0x3, 0x2, 0x2, 0x2, 0xf2, 0xf4, 
    0x5, 0xc, 0x7, 0x2, 0xf3, 0xe7, 0x3, 0x2, 0x2, 0x2, 0xf3, 0xee, 0x3, 
    0x2, 0x2, 0x2, 0xf4, 0xb, 0x3, 0x2, 0x2, 0x2, 0xf5, 0xf8, 0x5, 0xe, 
    0x8, 0x2, 0xf6, 0xf8, 0x5, 0x10, 0x9, 0x2, 0xf7, 0xf5, 0x3, 0x2, 0x2, 
    0x2, 0xf7, 0xf6, 0x3, 0x2, 0x2, 0x2, 0xf8, 0xd, 0x3, 0x2, 0x2, 0x2, 
    0xf9, 0xfb, 0x5, 0x14, 0xb, 0x2, 0xfa, 0xfc, 0x7, 0x7f, 0x2, 0x2, 0xfb, 
    0xfa, 0x3, 0x2, 0x2, 0x2, 0xfb, 0xfc, 0x3, 0x2, 0x2, 0x2, 0xfc, 0xfe, 
    0x3, 0x2, 0x2, 0x2, 0xfd, 0xf9, 0x3, 0x2, 0x2, 0x2, 0xfe, 0x101, 0x3, 
    0x2, 0x2, 0x2, 0xff, 0xfd, 0x3, 0x2, 0x2, 0x2, 0xff, 0x100, 0x3, 0x2, 
    0x2, 0x2, 0x100, 0x102, 0x3, 0x2, 0x2, 0x2, 0x101, 0xff, 0x3, 0x2, 0x2, 
    0x2, 0x102, 0x11d, 0x5, 0x34, 0x1b, 0x2, 0x103, 0x105, 0x5, 0x14, 0xb, 
    0x2, 0x104, 0x106, 0x7, 0x7f, 0x2, 0x2, 0x105, 0x104, 0x3, 0x2, 0x2, 
    0x2, 0x105, 0x106, 0x3, 0x2, 0x2, 0x2, 0x106, 0x108, 0x3, 0x2, 0x2, 
    0x2, 0x107, 0x103, 0x3, 0x2, 0x2, 0x2, 0x108, 0x10b, 0x3, 0x2, 0x2, 
    0x2, 0x109, 0x107, 0x3, 0x2, 0x2, 0x2, 0x109, 0x10a, 0x3, 0x2, 0x2, 
    0x2, 0x10a, 0x10c, 0x3, 0x2, 0x2, 0x2, 0x10b, 0x109, 0x3, 0x2, 0x2, 
    0x2, 0x10c, 0x113, 0x5, 0x12, 0xa, 0x2, 0x10d, 0x10f, 0x7, 0x7f, 0x2, 
    0x2, 0x10e, 0x10d, 0x3, 0x2, 0x2, 0x2, 0x10e, 0x10f, 0x3, 0x2, 0x2, 
    0x2, 0x10f, 0x110, 0x3, 0x2, 0x2, 0x2, 0x110, 0x112, 0x5, 0x12, 0xa, 
    0x2, 0x111, 0x10e, 0x3, 0x2, 0x2, 0x2, 0x112, 0x115, 0x3, 0x2, 0x2, 
    0x2, 0x113, 0x111, 0x3, 0x2, 0x2, 0x2, 0x113, 0x114, 0x3, 0x2, 0x2, 
    0x2, 0x114, 0x11a, 0x3, 0x2, 0x2, 0x2, 0x115, 0x113, 0x3, 0x2, 0x2, 
    0x2, 0x116, 0x118, 0x7, 0x7f, 0x2, 0x2, 0x117, 0x116, 0x3, 0x2, 0x2, 
    0x2, 0x117, 0x118, 0x3, 0x2, 0x2, 0x2, 0x118, 0x119, 0x3, 0x2, 0x2, 
    0x2, 0x119, 0x11b, 0x5, 0x34, 0x1b, 0x2, 0x11a, 0x117, 0x3, 0x2, 0x2, 
    0x2, 0x11a, 0x11b, 0x3, 0x2, 0x2, 0x2, 0x11b, 0x11d, 0x3, 0x2, 0x2, 
    0x2, 0x11c, 0xff, 0x3, 0x2, 0x2, 0x2, 0x11c, 0x109, 0x3, 0x2, 0x2, 0x2, 
    0x11d, 0xf, 0x3, 0x2, 0x2, 0x2, 0x11e, 0x120, 0x5, 0x14, 0xb, 0x2, 0x11f, 
    0x121, 0x7, 0x7f, 0x2, 0x2, 0x120, 0x11f, 0x3, 0x2, 0x2, 0x2, 0x120, 
    0x121, 0x3, 0x2, 0x2, 0x2, 0x121, 0x123, 0x3, 0x2, 0x2, 0x2, 0x122, 
    0x11e, 0x3, 0x2, 0x2, 0x2, 0x123, 0x126, 0x3, 0x2, 0x2, 0x2, 0x124, 
    0x122, 0x3, 0x2, 0x2, 0x2, 0x124, 0x125, 0x3, 0x2, 0x2, 0x2, 0x125, 
    0x12d, 0x3, 0x2, 0x2, 0x2, 0x126, 0x124, 0x3, 0x2, 0x2, 0x2, 0x127, 
    0x129, 0x5, 0x12, 0xa, 0x2, 0x128, 0x12a, 0x7, 0x7f, 0x2, 0x2, 0x129, 
    0x128, 0x3, 0x2, 0x2, 0x2, 0x129, 0x12a, 0x3, 0x2, 0x2, 0x2, 0x12a, 
    0x12c, 0x3, 0x2, 0x2, 0x2, 0x12b, 0x127, 0x3, 0x2, 0x2, 0x2, 0x12c, 
    0x12f, 0x3, 0x2, 0x2, 0x2, 0x12d, 0x12b, 0x3, 0x2, 0x2, 0x2, 0x12d, 
    0x12e, 0x3, 0x2, 0x2, 0x2, 0x12e, 0x130, 0x3, 0x2, 0x2, 0x2, 0x12f, 
    0x12d, 0x3, 0x2, 0x2, 0x2, 0x130, 0x132, 0x5, 0x32, 0x1a, 0x2, 0x131, 
    0x133, 0x7, 0x7f, 0x2, 0x2, 0x132, 0x131, 0x3, 0x2, 0x2, 0x2, 0x132, 
    0x133, 0x3, 0x2, 0x2, 0x2, 0x133, 0x135, 0x3, 0x2, 0x2, 0x2, 0x134, 
    0x124, 0x3, 0x2, 0x2, 0x2, 0x135, 0x136, 0x3, 0x2, 0x2, 0x2, 0x136, 
    0x134, 0x3, 0x2, 0x2, 0x2, 0x136, 0x137, 0x3, 0x2, 0x2, 0x2, 0x137, 
    0x138, 0x3, 0x2, 0x2, 0x2, 0x138, 0x139, 0x5, 0xe, 0x8, 0x2, 0x139, 
    0x11, 0x3, 0x2, 0x2, 0x2, 0x13a, 0x140, 0x5, 0x1e, 0x10, 0x2, 0x13b, 
    0x140, 0x5, 0x1a, 0xe, 0x2, 0x13c, 0x140, 0x5, 0x24, 0x13, 0x2, 0x13d, 
    0x140, 0x5, 0x20, 0x11, 0x2, 0x13e, 0x140, 0x5, 0x26, 0x14, 0x2, 0x13f, 
    0x13a, 0x3, 0x2, 0x2, 0x2, 0x13f, 0x13b, 0x3, 0x2, 0x2, 0x2, 0x13f, 
    0x13c, 0x3, 0x2, 0x2, 0x2, 0x13f, 0x13d, 0x3, 0x2, 0x2, 0x2, 0x13f, 
    0x13e, 0x3, 0x2, 0x2, 0x2, 0x140, 0x13, 0x3, 0x2, 0x2, 0x2, 0x141, 0x145, 
    0x5, 0x16, 0xc, 0x2, 0x142, 0x145, 0x5, 0x18, 0xd, 0x2, 0x143, 0x145, 
    0x5, 0x2a, 0x16, 0x2, 0x144, 0x141, 0x3, 0x2, 0x2, 0x2, 0x144, 0x142, 
    0x3, 0x2, 0x2, 0x2, 0x144, 0x143, 0x3, 0x2, 0x2, 0x2, 0x145, 0x15, 0x3, 
    0x2, 0x2, 0x2, 0x146, 0x147, 0x7, 0x32, 0x2, 0x2, 0x147, 0x149, 0x7, 
    0x7f, 0x2, 0x2, 0x148, 0x146, 0x3, 0x2, 0x2, 0x2, 0x148, 0x149, 0x3, 
    0x2, 0x2, 0x2, 0x149, 0x14a, 0x3, 0x2, 0x2, 0x2, 0x14a, 0x14c, 0x7, 
    0x33, 0x2, 0x2, 0x14b, 0x14d, 0x7, 0x7f, 0x2, 0x2, 0x14c, 0x14b, 0x3, 
    0x2, 0x2, 0x2, 0x14c, 0x14d, 0x3, 0x2, 0x2, 0x2, 0x14d, 0x14e, 0x3, 
    0x2, 0x2, 0x2, 0x14e, 0x153, 0x5, 0x46, 0x24, 0x2, 0x14f, 0x151, 0x7, 
    0x7f, 0x2, 0x2, 0x150, 0x14f, 0x3, 0x2, 0x2, 0x2, 0x150, 0x151, 0x3, 
    0x2, 0x2, 0x2, 0x151, 0x152, 0x3, 0x2, 0x2, 0x2, 0x152, 0x154, 0x5, 
    0x44, 0x23, 0x2, 0x153, 0x150, 0x3, 0x2, 0x2, 0x2, 0x153, 0x154, 0x3, 
    0x2, 0x2, 0x2, 0x154, 0x17, 0x3, 0x2, 0x2, 0x2, 0x155, 0x157, 0x7, 0x34, 
    0x2, 0x2, 0x156, 0x158, 0x7, 0x7f, 0x2, 0x2, 0x157, 0x156, 0x3, 0x2, 
    0x2, 0x2, 0x157, 0x158, 0x3, 0x2, 0x2, 0x2, 0x158, 0x159, 0x3, 0x2, 
    0x2, 0x2, 0x159, 0x15a, 0x5, 0x64, 0x33, 0x2, 0x15a, 0x15b, 0x7, 0x7f, 
    0x2, 0x2, 0x15b, 0x15c, 0x7, 0x35, 0x2, 0x2, 0x15c, 0x15d, 0x7, 0x7f, 
    0x2, 0x2, 0x15d, 0x15e, 0x5, 0xac, 0x57, 0x2, 0x15e, 0x19, 0x3, 0x2, 
    0x2, 0x2, 0x15f, 0x161, 0x7, 0x36, 0x2, 0x2, 0x160, 0x162, 0x7, 0x7f, 
    0x2, 0x2, 0x161, 0x160, 0x3, 0x2, 0x2, 0x2, 0x161, 0x162, 0x3, 0x2, 
    0x2, 0x2, 0x162, 0x163, 0x3, 0x2, 0x2, 0x2, 0x163, 0x168, 0x5, 0x48, 
    0x25, 0x2, 0x164, 0x165, 0x7, 0x7f, 0x2, 0x2, 0x165, 0x167, 0x5, 0x1c, 
    0xf, 0x2, 0x166, 0x164, 0x3, 0x2, 0x2, 0x2, 0x167, 0x16a, 0x3, 0x2, 
    0x2, 0x2, 0x168, 0x166, 0x3, 0x2, 0x2, 0x2, 0x168, 0x169, 0x3, 0x2, 
    0x2, 0x2, 0x169, 0x1b, 0x3, 0x2, 0x2, 0x2, 0x16a, 0x168, 0x3, 0x2, 0x2, 
    0x2, 0x16b, 0x16c, 0x7, 0x37, 0x2, 0x2, 0x16c, 0x16d, 0x7, 0x7f, 0x2, 
    0x2, 0x16d, 0x16e, 0x7, 0x33, 0x2, 0x2, 0x16e, 0x16f, 0x7, 0x7f, 0x2, 
    0x2, 0x16f, 0x176, 0x5, 0x20, 0x11, 0x2, 0x170, 0x171, 0x7, 0x37, 0x2, 
    0x2, 0x171, 0x172, 0x7, 0x7f, 0x2, 0x2, 0x172, 0x173, 0x7, 0x38, 0x2, 
    0x2, 0x173, 0x174, 0x7, 0x7f, 0x2, 0x2, 0x174, 0x176, 0x5, 0x20, 0x11, 
    0x2, 0x175, 0x16b, 0x3, 0x2, 0x2, 0x2, 0x175, 0x170, 0x3, 0x2, 0x2, 
    0x2, 0x176, 0x1d, 0x3, 0x2, 0x2, 0x2, 0x177, 0x179, 0x7, 0x38, 0x2, 
    0x2, 0x178, 0x17a, 0x7, 0x7f, 0x2, 0x2, 0x179, 0x178, 0x3, 0x2, 0x2, 
    0x2, 0x179, 0x17a, 0x3, 0x2, 0x2, 0x2, 0x17a, 0x17b, 0x3, 0x2, 0x2, 
    0x2, 0x17b, 0x17c, 0x5, 0x46, 0x24, 0x2, 0x17c, 0x1f, 0x3, 0x2, 0x2, 
    0x2, 0x17d, 0x17f, 0x7, 0x39, 0x2, 0x2, 0x17e, 0x180, 0x7, 0x7f, 0x2, 
    0x2, 0x17f, 0x17e, 0x3, 0x2, 0x2, 0x2, 0x17f, 0x180, 0x3, 0x2, 0x2, 
    0x2, 0x180, 0x181, 0x3, 0x2, 0x2, 0x2, 0x181, 0x186, 0x5, 0x22, 0x12, 
    0x2, 0x182, 0x183, 0x7, 0x4, 0x2, 0x2, 0x183, 0x185, 0x5, 0x22, 0x12, 
    0x2, 0x184, 0x182, 0x3, 0x2, 0x2, 0x2, 0x185, 0x188, 0x3, 0x2, 0x2, 
    0x2, 0x186, 0x184, 0x3, 0x2, 0x2, 0x2, 0x186, 0x187, 0x3, 0x2, 0x2, 
    0x2, 0x187, 0x21, 0x3, 0x2, 0x2, 0x2, 0x188, 0x186, 0x3, 0x2, 0x2, 0x2, 
    0x189, 0x18b, 0x5, 0xb4, 0x5b, 0x2, 0x18a, 0x18c, 0x7, 0x7f, 0x2, 0x2, 
    0x18b, 0x18a, 0x3, 0x2, 0x2, 0x2, 0x18b, 0x18c, 0x3, 0x2, 0x2, 0x2, 
    0x18c, 0x18d, 0x3, 0x2, 0x2, 0x2, 0x18d, 0x18f, 0x7, 0x5, 0x2, 0x2, 
    0x18e, 0x190, 0x7, 0x7f, 0x2, 0x2, 0x18f, 0x18e, 0x3, 0x2, 0x2, 0x2, 
    0x18f, 0x190, 0x3, 0x2, 0x2, 0x2, 0x190, 0x191, 0x3, 0x2, 0x2, 0x2, 
    0x191, 0x192, 0x5, 0x64, 0x33, 0x2, 0x192, 0x1ae, 0x3, 0x2, 0x2, 0x2, 
    0x193, 0x195, 0x5, 0xac, 0x57, 0x2, 0x194, 0x196, 0x7, 0x7f, 0x2, 0x2, 
    0x195, 0x194, 0x3, 0x2, 0x2, 0x2, 0x195, 0x196, 0x3, 0x2, 0x2, 0x2, 
    0x196, 0x197, 0x3, 0x2, 0x2, 0x2, 0x197, 0x199, 0x7, 0x5, 0x2, 0x2, 
    0x198, 0x19a, 0x7, 0x7f, 0x2, 0x2, 0x199, 0x198, 0x3, 0x2, 0x2, 0x2, 
    0x199, 0x19a, 0x3, 0x2, 0x2, 0x2, 0x19a, 0x19b, 0x3, 0x2, 0x2, 0x2, 
    0x19b, 0x19c, 0x5, 0x64, 0x33, 0x2, 0x19c, 0x1ae, 0x3, 0x2, 0x2, 0x2, 
    0x19d, 0x19f, 0x5, 0xac, 0x57, 0x2, 0x19e, 0x1a0, 0x7, 0x7f, 0x2, 0x2, 
    0x19f, 0x19e, 0x3, 0x2, 0x2, 0x2, 0x19f, 0x1a0, 0x3, 0x2, 0x2, 0x2, 
    0x1a0, 0x1a1, 0x3, 0x2, 0x2, 0x2, 0x1a1, 0x1a3, 0x7, 0x6, 0x2, 0x2, 
    0x1a2, 0x1a4, 0x7, 0x7f, 0x2, 0x2, 0x1a3, 0x1a2, 0x3, 0x2, 0x2, 0x2, 
    0x1a3, 0x1a4, 0x3, 0x2, 0x2, 0x2, 0x1a4, 0x1a5, 0x3, 0x2, 0x2, 0x2, 
    0x1a5, 0x1a6, 0x5, 0x64, 0x33, 0x2, 0x1a6, 0x1ae, 0x3, 0x2, 0x2, 0x2, 
    0x1a7, 0x1a9, 0x5, 0xac, 0x57, 0x2, 0x1a8, 0x1aa, 0x7, 0x7f, 0x2, 0x2, 
    0x1a9, 0x1a8, 0x3, 0x2, 0x2, 0x2, 0x1a9, 0x1aa, 0x3, 0x2, 0x2, 0x2, 
    0x1aa, 0x1ab, 0x3, 0x2, 0x2, 0x2, 0x1ab, 0x1ac, 0x5, 0x5a, 0x2e, 0x2, 
    0x1ac, 0x1ae, 0x3, 0x2, 0x2, 0x2, 0x1ad, 0x189, 0x3, 0x2, 0x2, 0x2, 
    0x1ad, 0x193, 0x3, 0x2, 0x2, 0x2, 0x1ad, 0x19d, 0x3, 0x2, 0x2, 0x2, 
    0x1ad, 0x1a7, 0x3, 0x2, 0x2, 0x2, 0x1ae, 0x23, 0x3, 0x2, 0x2, 0x2, 0x1af, 
    0x1b0, 0x7, 0x3a, 0x2, 0x2, 0x1b0, 0x1b2, 0x7, 0x7f, 0x2, 0x2, 0x1b1, 
    0x1af, 0x3, 0x2, 0x2, 0x2, 0x1b1, 0x1b2, 0x3, 0x2, 0x2, 0x2, 0x1b2, 
    0x1b3, 0x3, 0x2, 0x2, 0x2, 0x1b3, 0x1b5, 0x7, 0x3b, 0x2, 0x2, 0x1b4, 
    0x1b6, 0x7, 0x7f, 0x2, 0x2, 0x1b5, 0x1b4, 0x3, 0x2, 0x2, 0x2, 0x1b5, 
    0x1b6, 0x3, 0x2, 0x2, 0x2, 0x1b6, 0x1b7, 0x3, 0x2, 0x2, 0x2, 0x1b7, 
    0x1c2, 0x5, 0x64, 0x33, 0x2, 0x1b8, 0x1ba, 0x7, 0x7f, 0x2, 0x2, 0x1b9, 
    0x1b8, 0x3, 0x2, 0x2, 0x2, 0x1b9, 0x1ba, 0x3, 0x2, 0x2, 0x2, 0x1ba, 
    0x1bb, 0x3, 0x2, 0x2, 0x2, 0x1bb, 0x1bd, 0x7, 0x4, 0x2, 0x2, 0x1bc, 
    0x1be, 0x7, 0x7f, 0x2, 0x2, 0x1bd, 0x1bc, 0x3, 0x2, 0x2, 0x2, 0x1bd, 
    0x1be, 0x3, 0x2, 0x2, 0x2, 0x1be, 0x1bf, 0x3, 0x2, 0x2, 0x2, 0x1bf, 
    0x1c1, 0x5, 0x64, 0x33, 0x2, 0x1c0, 0x1b9, 0x3, 0x2, 0x2, 0x2, 0x1c1, 
    0x1c4, 0x3, 0x2, 0x2, 0x2, 0x1c2, 0x1c0, 0x3, 0x2, 0x2, 0x2, 0x1c2, 
    0x1c3, 0x3, 0x2, 0x2, 0x2, 0x1c3, 0x25, 0x3, 0x2, 0x2, 0x2, 0x1c4, 0x1c2, 
    0x3, 0x2, 0x2, 0x2, 0x1c5, 0x1c6, 0x7, 0x3c, 0x2, 0x2, 0x1c6, 0x1c7, 
    0x7, 0x7f, 0x2, 0x2, 0x1c7, 0x1d2, 0x5, 0x28, 0x15, 0x2, 0x1c8, 0x1ca, 
    0x7, 0x7f, 0x2, 0x2, 0x1c9, 0x1c8, 0x3, 0x2, 0x2, 0x2, 0x1c9, 0x1ca, 
    0x3, 0x2, 0x2, 0x2, 0x1ca, 0x1cb, 0x3, 0x2, 0x2, 0x2, 0x1cb, 0x1cd, 
    0x7, 0x4, 0x2, 0x2, 0x1cc, 0x1ce, 0x7, 0x7f, 0x2, 0x2, 0x1cd, 0x1cc, 
    0x3, 0x2, 0x2, 0x2, 0x1cd, 0x1ce, 0x3, 0x2, 0x2, 0x2, 0x1ce, 0x1cf, 
    0x3, 0x2, 0x2, 0x2, 0x1cf, 0x1d1, 0x5, 0x28, 0x15, 0x2, 0x1d0, 0x1c9, 
    0x3, 0x2, 0x2, 0x2, 0x1d1, 0x1d4, 0x3, 0x2, 0x2, 0x2, 0x1d2, 0x1d0, 
    0x3, 0x2, 0x2, 0x2, 0x1d2, 0x1d3, 0x3, 0x2, 0x2, 0x2, 0x1d3, 0x27, 0x3, 
    0x2, 0x2, 0x2, 0x1d4, 0x1d2, 0x3, 0x2, 0x2, 0x2, 0x1d5, 0x1d6, 0x5, 
    0xac, 0x57, 0x2, 0x1d6, 0x1d7, 0x5, 0x5a, 0x2e, 0x2, 0x1d7, 0x1da, 0x3, 
    0x2, 0x2, 0x2, 0x1d8, 0x1da, 0x5, 0xb4, 0x5b, 0x2, 0x1d9, 0x1d5, 0x3, 
    0x2, 0x2, 0x2, 0x1d9, 0x1d8, 0x3, 0x2, 0x2, 0x2, 0x1da, 0x29, 0x3, 0x2, 
    0x2, 0x2, 0x1db, 0x1dc, 0x7, 0x3d, 0x2, 0x2, 0x1dc, 0x1dd, 0x7, 0x7f, 
    0x2, 0x2, 0x1dd, 0x1e4, 0x5, 0x98, 0x4d, 0x2, 0x1de, 0x1e0, 0x7, 0x7f, 
    0x2, 0x2, 0x1df, 0x1de, 0x3, 0x2, 0x2, 0x2, 0x1df, 0x1e0, 0x3, 0x2, 
    0x2, 0x2, 0x1e0, 0x1e1, 0x3, 0x2, 0x2, 0x2, 0x1e1, 0x1e2, 0x7, 0x3e, 
    0x2, 0x2, 0x1e2, 0x1e3, 0x7, 0x7f, 0x2, 0x2, 0x1e3, 0x1e5, 0x5, 0x2e, 
    0x18, 0x2, 0x1e4, 0x1df, 0x3, 0x2, 0x2, 0x2, 0x1e4, 0x1e5, 0x3, 0x2, 
    0x2, 0x2, 0x1e5, 0x2b, 0x3, 0x2, 0x2, 0x2, 0x1e6, 0x1e7, 0x7, 0x3d, 
    0x2, 0x2, 0x1e7, 0x1ea, 0x7, 0x7f, 0x2, 0x2, 0x1e8, 0x1eb, 0x5, 0x98, 
    0x4d, 0x2, 0x1e9, 0x1eb, 0x5, 0x9a, 0x4e, 0x2, 0x1ea, 0x1e8, 0x3, 0x2, 
    0x2, 0x2, 0x1ea, 0x1e9, 0x3, 0x2, 0x2, 0x2, 0x1eb, 0x1f0, 0x3, 0x2, 
    0x2, 0x2, 0x1ec, 0x1ed, 0x7, 0x7f, 0x2, 0x2, 0x1ed, 0x1ee, 0x7, 0x3e, 
    0x2, 0x2, 0x1ee, 0x1ef, 0x7, 0x7f, 0x2, 0x2, 0x1ef, 0x1f1, 0x5, 0x2e, 
    0x18, 0x2, 0x1f0, 0x1ec, 0x3, 0x2, 0x2, 0x2, 0x1f0, 0x1f1, 0x3, 0x2, 
    0x2, 0x2, 0x1f1, 0x2d, 0x3, 0x2, 0x2, 0x2, 0x1f2, 0x202, 0x7, 0x7, 0x2, 
    0x2, 0x1f3, 0x1fe, 0x5, 0x30, 0x19, 0x2, 0x1f4, 0x1f6, 0x7, 0x7f, 0x2, 
    0x2, 0x1f5, 0x1f4, 0x3, 0x2, 0x2, 0x2, 0x1f5, 0x1f6, 0x3, 0x2, 0x2, 
    0x2, 0x1f6, 0x1f7, 0x3, 0x2, 0x2, 0x2, 0x1f7, 0x1f9, 0x7, 0x4, 0x2, 
    0x2, 0x1f8, 0x1fa, 0x7, 0x7f, 0x2, 0x2, 0x1f9, 0x1f8, 0x3, 0x2, 0x2, 
    0x2, 0x1f9, 0x1fa, 0x3, 0x2, 0x2, 0x2, 0x1fa, 0x1fb, 0x3, 0x2, 0x2, 
    0x2, 0x1fb, 0x1fd, 0x5, 0x30, 0x19, 0x2, 0x1fc, 0x1f5, 0x3, 0x2, 0x2, 
    0x2, 0x1fd, 0x200, 0x3, 0x2, 0x2, 0x2, 0x1fe, 0x1fc, 0x3, 0x2, 0x2, 
    0x2, 0x1fe, 0x1ff, 0x3, 0x2, 0x2, 0x2, 0x1ff, 0x202, 0x3, 0x2, 0x2, 
    0x2, 0x200, 0x1fe, 0x3, 0x2, 0x2, 0x2, 0x201, 0x1f2, 0x3, 0x2, 0x2, 
    0x2, 0x201, 0x1f3, 0x3, 0x2, 0x2, 0x2, 0x202, 0x207, 0x3, 0x2, 0x2, 
    0x2, 0x203, 0x205, 0x7, 0x7f, 0x2, 0x2, 0x204, 0x203, 0x3, 0x2, 0x2, 
    0x2, 0x204, 0x205, 0x3, 0x2, 0x2, 0x2, 0x205, 0x206, 0x3, 0x2, 0x2, 
    0x2, 0x206, 0x208, 0x5, 0x44, 0x23, 0x2, 0x207, 0x204, 0x3, 0x2, 0x2, 
    0x2, 0x207, 0x208, 0x3, 0x2, 0x2, 0x2, 0x208, 0x2f, 0x3, 0x2, 0x2, 0x2, 
    0x209, 0x20a, 0x5, 0x9c, 0x4f, 0x2, 0x20a, 0x20b, 0x7, 0x7f, 0x2, 0x2, 
    0x20b, 0x20c, 0x7, 0x35, 0x2, 0x2, 0x20c, 0x20d, 0x7, 0x7f, 0x2, 0x2, 
    0x20d, 0x20f, 0x3, 0x2, 0x2, 0x2, 0x20e, 0x209, 0x3, 0x2, 0x2, 0x2, 
    0x20e, 0x20f, 0x3, 0x2, 0x2, 0x2, 0x20f, 0x210, 0x3, 0x2, 0x2, 0x2, 
    0x210, 0x211, 0x5, 0xac, 0x57, 0x2, 0x211, 0x31, 0x3, 0x2, 0x2, 0x2, 
    0x212, 0x217, 0x7, 0x3f, 0x2, 0x2, 0x213, 0x215, 0x7, 0x7f, 0x2, 0x2, 
    0x214, 0x213, 0x3, 0x2, 0x2, 0x2, 0x214, 0x215, 0x3, 0x2, 0x2, 0x2, 
    0x215, 0x216, 0x3, 0x2, 0x2, 0x2, 0x216, 0x218, 0x7, 0x40, 0x2, 0x2, 
    0x217, 0x214, 0x3, 0x2, 0x2, 0x2, 0x217, 0x218, 0x3, 0x2, 0x2, 0x2, 
    0x218, 0x219, 0x3, 0x2, 0x2, 0x2, 0x219, 0x21a, 0x7, 0x7f, 0x2, 0x2, 
    0x21a, 0x21f, 0x5, 0x36, 0x1c, 0x2, 0x21b, 0x21d, 0x7, 0x7f, 0x2, 0x2, 
    0x21c, 0x21b, 0x3, 0x2, 0x2, 0x2, 0x21c, 0x21d, 0x3, 0x2, 0x2, 0x2, 
    0x21d, 0x21e, 0x3, 0x2, 0x2, 0x2, 0x21e, 0x220, 0x5, 0x44, 0x23, 0x2, 
    0x21f, 0x21c, 0x3, 0x2, 0x2, 0x2, 0x21f, 0x220, 0x3, 0x2, 0x2, 0x2, 
    0x220, 0x33, 0x3, 0x2, 0x2, 0x2, 0x221, 0x226, 0x7, 0x41, 0x2, 0x2, 
    0x222, 0x224, 0x7, 0x7f, 0x2, 0x2, 0x223, 0x222, 0x3, 0x2, 0x2, 0x2, 
    0x223, 0x224, 0x3, 0x2, 0x2, 0x2, 0x224, 0x225, 0x3, 0x2, 0x2, 0x2, 
    0x225, 0x227, 0x7, 0x40, 0x2, 0x2, 0x226, 0x223, 0x3, 0x2, 0x2, 0x2, 
    0x226, 0x227, 0x3, 0x2, 0x2, 0x2, 0x227, 0x228, 0x3, 0x2, 0x2, 0x2, 
    0x228, 0x229, 0x7, 0x7f, 0x2, 0x2, 0x229, 0x22a, 0x5, 0x36, 0x1c, 0x2, 
    0x22a, 0x35, 0x3, 0x2, 0x2, 0x2, 0x22b, 0x22e, 0x5, 0x38, 0x1d, 0x2, 
    0x22c, 0x22d, 0x7, 0x7f, 0x2, 0x2, 0x22d, 0x22f, 0x5, 0x3c, 0x1f, 0x2, 
    0x22e, 0x22c, 0x3, 0x2, 0x2, 0x2, 0x22e, 0x22f, 0x3, 0x2, 0x2, 0x2, 
    0x22f, 0x232, 0x3, 0x2, 0x2, 0x2, 0x230, 0x231, 0x7, 0x7f, 0x2, 0x2, 
    0x231, 0x233, 0x5, 0x3e, 0x20, 0x2, 0x232, 0x230, 0x3, 0x2, 0x2, 0x2, 
    0x232, 0x233, 0x3, 0x2, 0x2, 0x2, 0x233, 0x236, 0x3, 0x2, 0x2, 0x2, 
    0x234, 0x235, 0x7, 0x7f, 0x2, 0x2, 0x235, 0x237, 0x5, 0x40, 0x21, 0x2, 
    0x236, 0x234, 0x3, 0x2, 0x2, 0x2, 0x236, 0x237, 0x3, 0x2, 0x2, 0x2, 
    0x237, 0x37, 0x3, 0x2, 0x2, 0x2, 0x238, 0x243, 0x7, 0x7, 0x2, 0x2, 0x239, 
    0x23b, 0x7, 0x7f, 0x2, 0x2, 0x23a, 0x239, 0x3, 0x2, 0x2, 0x2, 0x23a, 
    0x23b, 0x3, 0x2, 0x2, 0x2, 0x23b, 0x23c, 0x3, 0x2, 0x2, 0x2, 0x23c, 
    0x23e, 0x7, 0x4, 0x2, 0x2, 0x23d, 0x23f, 0x7, 0x7f, 0x2, 0x2, 0x23e, 
    0x23d, 0x3, 0x2, 0x2, 0x2, 0x23e, 0x23f, 0x3, 0x2, 0x2, 0x2, 0x23f, 
    0x240, 0x3, 0x2, 0x2, 0x2, 0x240, 0x242, 0x5, 0x3a, 0x1e, 0x2, 0x241, 
    0x23a, 0x3, 0x2, 0x2, 0x2, 0x242, 0x245, 0x3, 0x2, 0x2, 0x2, 0x243, 
    0x241, 0x3, 0x2, 0x2, 0x2, 0x243, 0x244, 0x3, 0x2, 0x2, 0x2, 0x244, 
    0x255, 0x3, 0x2, 0x2, 0x2, 0x245, 0x243, 0x3, 0x2, 0x2, 0x2, 0x246, 
    0x251, 0x5, 0x3a, 0x1e, 0x2, 0x247, 0x249, 0x7, 0x7f, 0x2, 0x2, 0x248, 
    0x247, 0x3, 0x2, 0x2, 0x2, 0x248, 0x249, 0x3, 0x2, 0x2, 0x2, 0x249, 
    0x24a, 0x3, 0x2, 0x2, 0x2, 0x24a, 0x24c, 0x7, 0x4, 0x2, 0x2, 0x24b, 
    0x24d, 0x7, 0x7f, 0x2, 0x2, 0x24c, 0x24b, 0x3, 0x2, 0x2, 0x2, 0x24c, 
    0x24d, 0x3, 0x2, 0x2, 0x2, 0x24d, 0x24e, 0x3, 0x2, 0x2, 0x2, 0x24e, 
    0x250, 0x5, 0x3a, 0x1e, 0x2, 0x24f, 0x248, 0x3, 0x2, 0x2, 0x2, 0x250, 
    0x253, 0x3, 0x2, 0x2, 0x2, 0x251, 0x24f, 0x3, 0x2, 0x2, 0x2, 0x251, 
    0x252, 0x3, 0x2, 0x2, 0x2, 0x252, 0x255, 0x3, 0x2, 0x2, 0x2, 0x253, 
    0x251, 0x3, 0x2, 0x2, 0x2, 0x254, 0x238, 0x3, 0x2, 0x2, 0x2, 0x254, 
    0x246, 0x3, 0x2, 0x2, 0x2, 0x255, 0x39, 0x3, 0x2, 0x2, 0x2, 0x256, 0x257, 
    0x5, 0x64, 0x33, 0x2, 0x257, 0x258, 0x7, 0x7f, 0x2, 0x2, 0x258, 0x259, 
    0x7, 0x35, 0x2, 0x2, 0x259, 0x25a, 0x7, 0x7f, 0x2, 0x2, 0x25a, 0x25b, 
    0x5, 0xac, 0x57, 0x2, 0x25b, 0x25e, 0x3, 0x2, 0x2, 0x2, 0x25c, 0x25e, 
    0x5, 0x64, 0x33, 0x2, 0x25d, 0x256, 0x3, 0x2, 0x2, 0x2, 0x25d, 0x25c, 
    0x3, 0x2, 0x2, 0x2, 0x25e, 0x3b, 0x3, 0x2, 0x2, 0x2, 0x25f, 0x260, 0x7, 
    0x42, 0x2, 0x2, 0x260, 0x261, 0x7, 0x7f, 0x2, 0x2, 0x261, 0x262, 0x7, 
    0x43, 0x2, 0x2, 0x262, 0x263, 0x7, 0x7f, 0x2, 0x2, 0x263, 0x26b, 0x5, 
    0x42, 0x22, 0x2, 0x264, 0x266, 0x7, 0x4, 0x2, 0x2, 0x265, 0x267, 0x7, 
    0x7f, 0x2, 0x2, 0x266, 0x265, 0x3, 0x2, 0x2, 0x2, 0x266, 0x267, 0x3, 
    0x2, 0x2, 0x2, 0x267, 0x268, 0x3, 0x2, 0x2, 0x2, 0x268, 0x26a, 0x5, 
    0x42, 0x22, 0x2, 0x269, 0x264, 0x3, 0x2, 0x2, 0x2, 0x26a, 0x26d, 0x3, 
    0x2, 0x2, 0x2, 0x26b, 0x269, 0x3, 0x2, 0x2, 0x2, 0x26b, 0x26c, 0x3, 
    0x2, 0x2, 0x2, 0x26c, 0x3d, 0x3, 0x2, 0x2, 0x2, 0x26d, 0x26b, 0x3, 0x2, 
    0x2, 0x2, 0x26e, 0x26f, 0x7, 0x44, 0x2, 0x2, 0x26f, 0x270, 0x7, 0x7f, 
    0x2, 0x2, 0x270, 0x271, 0x5, 0x64, 0x33, 0x2, 0x271, 0x3f, 0x3, 0x2, 
    0x2, 0x2, 0x272, 0x273, 0x7, 0x45, 0x2, 0x2, 0x273, 0x274, 0x7, 0x7f, 
    0x2, 0x2, 0x274, 0x275, 0x5, 0x64, 0x33, 0x2, 0x275, 0x41, 0x3, 0x2, 
    0x2, 0x2, 0x276, 0x27b, 0x5, 0x64, 0x33, 0x2, 0x277, 0x279, 0x7, 0x7f, 
    0x2, 0x2, 0x278, 0x277, 0x3, 0x2, 0x2, 0x2, 0x278, 0x279, 0x3, 0x2, 
    0x2, 0x2, 0x279, 0x27a, 0x3, 0x2, 0x2, 0x2, 0x27a, 0x27c, 0x9, 0x2, 
    0x2, 0x2, 0x27b, 0x278, 0x3, 0x2, 0x2, 0x2, 0x27b, 0x27c, 0x3, 0x2, 
    0x2, 0x2, 0x27c, 0x43, 0x3, 0x2, 0x2, 0x2, 0x27d, 0x27e, 0x7, 0x4a, 
    0x2, 0x2, 0x27e, 0x27f, 0x7, 0x7f, 0x2, 0x2, 0x27f, 0x280, 0x5, 0x64, 
    0x33, 0x2, 0x280, 0x45, 0x3, 0x2, 0x2, 0x2, 0x281, 0x28c, 0x5, 0x48, 
    0x25, 0x2, 0x282, 0x284, 0x7, 0x7f, 0x2, 0x2, 0x283, 0x282, 0x3, 0x2, 
    0x2, 0x2, 0x283, 0x284, 0x3, 0x2, 0x2, 0x2, 0x284, 0x285, 0x3, 0x2, 
    0x2, 0x2, 0x285, 0x287, 0x7, 0x4, 0x2, 0x2, 0x286, 0x288, 0x7, 0x7f, 
    0x2, 0x2, 0x287, 0x286, 0x3, 0x2, 0x2, 0x2, 0x287, 0x288, 0x3, 0x2, 
    0x2, 0x2, 0x288, 0x289, 0x3, 0x2, 0x2, 0x2, 0x289, 0x28b, 0x5, 0x48, 
    0x25, 0x2, 0x28a, 0x283, 0x3, 0x2, 0x2, 0x2, 0x28b, 0x28e, 0x3, 0x2, 
    0x2, 0x2, 0x28c, 0x28a, 0x3, 0x2, 0x2, 0x2, 0x28c, 0x28d, 0x3, 0x2, 
    0x2, 0x2, 0x28d, 0x47, 0x3, 0x2, 0x2, 0x2, 0x28e, 0x28c, 0x3, 0x2, 0x2, 
    0x2, 0x28f, 0x291, 0x5, 0xac, 0x57, 0x2, 0x290, 0x292, 0x7, 0x7f, 0x2, 
    0x2, 0x291, 0x290, 0x3, 0x2, 0x2, 0x2, 0x291, 0x292, 0x3, 0x2, 0x2, 
    0x2, 0x292, 0x293, 0x3, 0x2, 0x2, 0x2, 0x293, 0x295, 0x7, 0x5, 0x2, 
    0x2, 0x294, 0x296, 0x7, 0x7f, 0x2, 0x2, 0x295, 0x294, 0x3, 0x2, 0x2, 
    0x2, 0x295, 0x296, 0x3, 0x2, 0x2, 0x2, 0x296, 0x297, 0x3, 0x2, 0x2, 
    0x2, 0x297, 0x298, 0x5, 0x4a, 0x26, 0x2, 0x298, 0x29b, 0x3, 0x2, 0x2, 
    0x2, 0x299, 0x29b, 0x5, 0x4a, 0x26, 0x2, 0x29a, 0x28f, 0x3, 0x2, 0x2, 
    0x2, 0x29a, 0x299, 0x3, 0x2, 0x2, 0x2, 0x29b, 0x49, 0x3, 0x2, 0x2, 0x2, 
    0x29c, 0x29d, 0x5, 0x4c, 0x27, 0x2, 0x29d, 0x4b, 0x3, 0x2, 0x2, 0x2, 
    0x29e, 0x2a5, 0x5, 0x4e, 0x28, 0x2, 0x29f, 0x2a1, 0x7, 0x7f, 0x2, 0x2, 
    0x2a0, 0x29f, 0x3, 0x2, 0x2, 0x2, 0x2a0, 0x2a1, 0x3, 0x2, 0x2, 0x2, 
    0x2a1, 0x2a2, 0x3, 0x2, 0x2, 0x2, 0x2a2, 0x2a4, 0x5, 0x50, 0x29, 0x2, 
    0x2a3, 0x2a0, 0x3, 0x2, 0x2, 0x2, 0x2a4, 0x2a7, 0x3, 0x2, 0x2, 0x2, 
    0x2a5, 0x2a3, 0x3, 0x2, 0x2, 0x2, 0x2a5, 0x2a6, 0x3, 0x2, 0x2, 0x2, 
    0x2a6, 0x2ad, 0x3, 0x2, 0x2, 0x2, 0x2a7, 0x2a5, 0x3, 0x2, 0x2, 0x2, 
    0x2a8, 0x2a9, 0x7, 0x8, 0x2, 0x2, 0x2a9, 0x2aa, 0x5, 0x4c, 0x27, 0x2, 
    0x2aa, 0x2ab, 0x7, 0x9, 0x2, 0x2, 0x2ab, 0x2ad, 0x3, 0x2, 0x2, 0x2, 
    0x2ac, 0x29e, 0x3, 0x2, 0x2, 0x2, 0x2ac, 0x2a8, 0x3, 0x2, 0x2, 0x2, 
    0x2ad, 0x4d, 0x3, 0x2, 0x2, 0x2, 0x2ae, 0x2b0, 0x7, 0x8, 0x2, 0x2, 0x2af, 
    0x2b1, 0x7, 0x7f, 0x2, 0x2, 0x2b0, 0x2af, 0x3, 0x2, 0x2, 0x2, 0x2b0, 
    0x2b1, 0x3, 0x2, 0x2, 0x2, 0x2b1, 0x2b6, 0x3, 0x2, 0x2, 0x2, 0x2b2, 
    0x2b4, 0x5, 0xac, 0x57, 0x2, 0x2b3, 0x2b5, 0x7, 0x7f, 0x2, 0x2, 0x2b4, 
    0x2b3, 0x3, 0x2, 0x2, 0x2, 0x2b4, 0x2b5, 0x3, 0x2, 0x2, 0x2, 0x2b5, 
    0x2b7, 0x3, 0x2, 0x2, 0x2, 0x2b6, 0x2b2, 0x3, 0x2, 0x2, 0x2, 0x2b6, 
    0x2b7, 0x3, 0x2, 0x2, 0x2, 0x2b7, 0x2bc, 0x3, 0x2, 0x2, 0x2, 0x2b8, 
    0x2ba, 0x5, 0x5a, 0x2e, 0x2, 0x2b9, 0x2bb, 0x7, 0x7f, 0x2, 0x2, 0x2ba, 
    0x2b9, 0x3, 0x2, 0x2, 0x2, 0x2ba, 0x2bb, 0x3, 0x2, 0x2, 0x2, 0x2bb, 
    0x2bd, 0x3, 0x2, 0x2, 0x2, 0x2bc, 0x2b8, 0x3, 0x2, 0x2, 0x2, 0x2bc, 
    0x2bd, 0x3, 0x2, 0x2, 0x2, 0x2bd, 0x2c2, 0x3, 0x2, 0x2, 0x2, 0x2be, 
    0x2c0, 0x5, 0x56, 0x2c, 0x2, 0x2bf, 0x2c1, 0x7, 0x7f, 0x2, 0x2, 0x2c0, 
    0x2bf, 0x3, 0x2, 0x2, 0x2, 0x2c0, 0x2c1, 0x3, 0x2, 0x2, 0x2, 0x2c1, 
    0x2c3, 0x3, 0x2, 0x2, 0x2, 0x2c2, 0x2be, 0x3, 0x2, 0x2, 0x2, 0x2c2, 
    0x2c3, 0x3, 0x2, 0x2, 0x2, 0x2c3, 0x2c4, 0x3, 0x2, 0x2, 0x2, 0x2c4, 
    0x2c5, 0x7, 0x9, 0x2, 0x2, 0x2c5, 0x4f, 0x3, 0x2, 0x2, 0x2, 0x2c6, 0x2c8, 
    0x5, 0x52, 0x2a, 0x2, 0x2c7, 0x2c9, 0x7, 0x7f, 0x2, 0x2, 0x2c8, 0x2c7, 
    0x3, 0x2, 0x2, 0x2, 0x2c8, 0x2c9, 0x3, 0x2, 0x2, 0x2, 0x2c9, 0x2ca, 
    0x3, 0x2, 0x2, 0x2, 0x2ca, 0x2cb, 0x5, 0x4e, 0x28, 0x2, 0x2cb, 0x51, 
    0x3, 0x2, 0x2, 0x2, 0x2cc, 0x2ce, 0x5, 0xc2, 0x62, 0x2, 0x2cd, 0x2cf, 
    0x7, 0x7f, 0x2, 0x2, 0x2ce, 0x2cd, 0x3, 0x2, 0x2, 0x2, 0x2ce, 0x2cf, 
    0x3, 0x2, 0x2, 0x2, 0x2cf, 0x2d0, 0x3, 0x2, 0x2, 0x2, 0x2d0, 0x2d2, 
    0x5, 0xc6, 0x64, 0x2, 0x2d1, 0x2d3, 0x7, 0x7f, 0x2, 0x2, 0x2d2, 0x2d1, 
    0x3, 0x2, 0x2, 0x2, 0x2d2, 0x2d3, 0x3, 0x2, 0x2, 0x2, 0x2d3, 0x2d5, 
    0x3, 0x2, 0x2, 0x2, 0x2d4, 0x2d6, 0x5, 0x54, 0x2b, 0x2, 0x2d5, 0x2d4, 
    0x3, 0x2, 0x2, 0x2, 0x2d5, 0x2d6, 0x3, 0x2, 0x2, 0x2, 0x2d6, 0x2d8, 
    0x3, 0x2, 0x2, 0x2, 0x2d7, 0x2d9, 0x7, 0x7f, 0x2, 0x2, 0x2d8, 0x2d7, 
    0x3, 0x2, 0x2, 0x2, 0x2d8, 0x2d9, 0x3, 0x2, 0x2, 0x2, 0x2d9, 0x2da, 
    0x3, 0x2, 0x2, 0x2, 0x2da, 0x2dc, 0x5, 0xc6, 0x64, 0x2, 0x2db, 0x2dd, 
    0x7, 0x7f, 0x2, 0x2, 0x2dc, 0x2db, 0x3, 0x2, 0x2, 0x2, 0x2dc, 0x2dd, 
    0x3, 0x2, 0x2, 0x2, 0x2dd, 0x2de, 0x3, 0x2, 0x2, 0x2, 0x2de, 0x2df, 
    0x5, 0xc4, 0x63, 0x2, 0x2df, 0x30d, 0x3, 0x2, 0x2, 0x2, 0x2e0, 0x2e2, 
    0x5, 0xc2, 0x62, 0x2, 0x2e1, 0x2e3, 0x7, 0x7f, 0x2, 0x2, 0x2e2, 0x2e1, 
    0x3, 0x2, 0x2, 0x2, 0x2e2, 0x2e3, 0x3, 0x2, 0x2, 0x2, 0x2e3, 0x2e4, 
    0x3, 0x2, 0x2, 0x2, 0x2e4, 0x2e6, 0x5, 0xc6, 0x64, 0x2, 0x2e5, 0x2e7, 
    0x7, 0x7f, 0x2, 0x2, 0x2e6, 0x2e5, 0x3, 0x2, 0x2, 0x2, 0x2e6, 0x2e7, 
    0x3, 0x2, 0x2, 0x2, 0x2e7, 0x2e9, 0x3, 0x2, 0x2, 0x2, 0x2e8, 0x2ea, 
    0x5, 0x54, 0x2b, 0x2, 0x2e9, 0x2e8, 0x3, 0x2, 0x2, 0x2, 0x2e9, 0x2ea, 
    0x3, 0x2, 0x2, 0x2, 0x2ea, 0x2ec, 0x3, 0x2, 0x2, 0x2, 0x2eb, 0x2ed, 
    0x7, 0x7f, 0x2, 0x2, 0x2ec, 0x2eb, 0x3, 0x2, 0x2, 0x2, 0x2ec, 0x2ed, 
    0x3, 0x2, 0x2, 0x2, 0x2ed, 0x2ee, 0x3, 0x2, 0x2, 0x2, 0x2ee, 0x2ef, 
    0x5, 0xc6, 0x64, 0x2, 0x2ef, 0x30d, 0x3, 0x2, 0x2, 0x2, 0x2f0, 0x2f2, 
    0x5, 0xc6, 0x64, 0x2, 0x2f1, 0x2f3, 0x7, 0x7f, 0x2, 0x2, 0x2f2, 0x2f1, 
    0x3, 0x2, 0x2, 0x2, 0x2f2, 0x2f3, 0x3, 0x2, 0x2, 0x2, 0x2f3, 0x2f5, 
    0x3, 0x2, 0x2, 0x2, 0x2f4, 0x2f6, 0x5, 0x54, 0x2b, 0x2, 0x2f5, 0x2f4, 
    0x3, 0x2, 0x2, 0x2, 0x2f5, 0x2f6, 0x3, 0x2, 0x2, 0x2, 0x2f6, 0x2f8, 
    0x3, 0x2, 0x2, 0x2, 0x2f7, 0x2f9, 0x7, 0x7f, 0x2, 0x2, 0x2f8, 0x2f7, 
    0x3, 0x2, 0x2, 0x2, 0x2f8, 0x2f9, 0x3, 0x2, 0x2, 0x2, 0x2f9, 0x2fa, 
    0x3, 0x2, 0x2, 0x2, 0x2fa, 0x2fc, 0x5, 0xc6, 0x64, 0x2, 0x2fb, 0x2fd, 
    0x7, 0x7f, 0x2, 0x2, 0x2fc, 0x2fb, 0x3, 0x2, 0x2, 0x2, 0x2fc, 0x2fd, 
    0x3, 0x2, 0x2, 0x2, 0x2fd, 0x2fe, 0x3, 0x2, 0x2, 0x2, 0x2fe, 0x2ff, 
    0x5, 0xc4, 0x63, 0x2, 0x2ff, 0x30d, 0x3, 0x2, 0x2, 0x2, 0x300, 0x302, 
    0x5, 0xc6, 0x64, 0x2, 0x301, 0x303, 0x7, 0x7f, 0x2, 0x2, 0x302, 0x301, 
    0x3, 0x2, 0x2, 0x2, 0x302, 0x303, 0x3, 0x2, 0x2, 0x2, 0x303, 0x305, 
    0x3, 0x2, 0x2, 0x2, 0x304, 0x306, 0x5, 0x54, 0x2b, 0x2, 0x305, 0x304, 
    0x3, 0x2, 0x2, 0x2, 0x305, 0x306, 0x3, 0x2, 0x2, 0x2, 0x306, 0x308, 
    0x3, 0x2, 0x2, 0x2, 0x307, 0x309, 0x7, 0x7f, 0x2, 0x2, 0x308, 0x307, 
    0x3, 0x2, 0x2, 0x2, 0x308, 0x309, 0x3, 0x2, 0x2, 0x2, 0x309, 0x30a, 
    0x3, 0x2, 0x2, 0x2, 0x30a, 0x30b, 0x5, 0xc6, 0x64, 0x2, 0x30b, 0x30d, 
    0x3, 0x2, 0x2, 0x2, 0x30c, 0x2cc, 0x3, 0x2, 0x2, 0x2, 0x30c, 0x2e0, 
    0x3, 0x2, 0x2, 0x2, 0x30c, 0x2f0, 0x3, 0x2, 0x2, 0x2, 0x30c, 0x300, 
    0x3, 0x2, 0x2, 0x2, 0x30d, 0x53, 0x3, 0x2, 0x2, 0x2, 0x30e, 0x310, 0x7, 
    0xa, 0x2, 0x2, 0x30f, 0x311, 0x7, 0x7f, 0x2, 0x2, 0x310, 0x30f, 0x3, 
    0x2, 0x2, 0x2, 0x310, 0x311, 0x3, 0x2, 0x2, 0x2, 0x311, 0x316, 0x3, 
    0x2, 0x2, 0x2, 0x312, 0x314, 0x5, 0xac, 0x57, 0x2, 0x313, 0x315, 0x7, 
    0x7f, 0x2, 0x2, 0x314, 0x313, 0x3, 0x2, 0x2, 0x2, 0x314, 0x315, 0x3, 
    0x2, 0x2, 0x2, 0x315, 0x317, 0x3, 0x2, 0x2, 0x2, 0x316, 0x312, 0x3, 
    0x2, 0x2, 0x2, 0x316, 0x317, 0x3, 0x2, 0x2, 0x2, 0x317, 0x31c, 0x3, 
    0x2, 0x2, 0x2, 0x318, 0x31a, 0x5, 0x58, 0x2d, 0x2, 0x319, 0x31b, 0x7, 
    0x7f, 0x2, 0x2, 0x31a, 0x319, 0x3, 0x2, 0x2, 0x2, 0x31a, 0x31b, 0x3, 
    0x2, 0x2, 0x2, 0x31b, 0x31d, 0x3, 0x2, 0x2, 0x2, 0x31c, 0x318, 0x3, 
    0x2, 0x2, 0x2, 0x31c, 0x31d, 0x3, 0x2, 0x2, 0x2, 0x31d, 0x31f, 0x3, 
    0x2, 0x2, 0x2, 0x31e, 0x320, 0x5, 0x5e, 0x30, 0x2, 0x31f, 0x31e, 0x3, 
    0x2, 0x2, 0x2, 0x31f, 0x320, 0x3, 0x2, 0x2, 0x2, 0x320, 0x325, 0x3, 
    0x2, 0x2, 0x2, 0x321, 0x323, 0x5, 0x56, 0x2c, 0x2, 0x322, 0x324, 0x7, 
    0x7f, 0x2, 0x2, 0x323, 0x322, 0x3, 0x2, 0x2, 0x2, 0x323, 0x324, 0x3, 
    0x2, 0x2, 0x2, 0x324, 0x326, 0x3, 0x2, 0x2, 0x2, 0x325, 0x321, 0x3, 
    0x2, 0x2, 0x2, 0x325, 0x326, 0x3, 0x2, 0x2, 0x2, 0x326, 0x327, 0x3, 
    0x2, 0x2, 0x2, 0x327, 0x328, 0x7, 0xb, 0x2, 0x2, 0x328, 0x55, 0x3, 0x2, 
    0x2, 0x2, 0x329, 0x32c, 0x5, 0xb0, 0x59, 0x2, 0x32a, 0x32c, 0x5, 0xb2, 
    0x5a, 0x2, 0x32b, 0x329, 0x3, 0x2, 0x2, 0x2, 0x32b, 0x32a, 0x3, 0x2, 
    0x2, 0x2, 0x32c, 0x57, 0x3, 0x2, 0x2, 0x2, 0x32d, 0x32f, 0x7, 0xc, 0x2, 
    0x2, 0x32e, 0x330, 0x7, 0x7f, 0x2, 0x2, 0x32f, 0x32e, 0x3, 0x2, 0x2, 
    0x2, 0x32f, 0x330, 0x3, 0x2, 0x2, 0x2, 0x330, 0x331, 0x3, 0x2, 0x2, 
    0x2, 0x331, 0x33f, 0x5, 0x62, 0x32, 0x2, 0x332, 0x334, 0x7, 0x7f, 0x2, 
    0x2, 0x333, 0x332, 0x3, 0x2, 0x2, 0x2, 0x333, 0x334, 0x3, 0x2, 0x2, 
    0x2, 0x334, 0x335, 0x3, 0x2, 0x2, 0x2, 0x335, 0x337, 0x7, 0xd, 0x2, 
    0x2, 0x336, 0x338, 0x7, 0xc, 0x2, 0x2, 0x337, 0x336, 0x3, 0x2, 0x2, 
    0x2, 0x337, 0x338, 0x3, 0x2, 0x2, 0x2, 0x338, 0x33a, 0x3, 0x2, 0x2, 
    0x2, 0x339, 0x33b, 0x7, 0x7f, 0x2, 0x2, 0x33a, 0x339, 0x3, 0x2, 0x2, 
    0x2, 0x33a, 0x33b, 0x3, 0x2, 0x2, 0x2, 0x33b, 0x33c, 0x3, 0x2, 0x2, 
    0x2, 0x33c, 0x33e, 0x5, 0x62, 0x32, 0x2, 0x33d, 0x333, 0x3, 0x2, 0x2, 
    0x2, 0x33e, 0x341, 0x3, 0x2, 0x2, 0x2, 0x33f, 0x33d, 0x3, 0x2, 0x2, 
    0x2, 0x33f, 0x340, 0x3, 0x2, 0x2, 0x2, 0x340, 0x59, 0x3, 0x2, 0x2, 0x2, 
    0x341, 0x33f, 0x3, 0x2, 0x2, 0x2, 0x342, 0x349, 0x5, 0x5c, 0x2f, 0x2, 
    0x343, 0x345, 0x7, 0x7f, 0x2, 0x2, 0x344, 0x343, 0x3, 0x2, 0x2, 0x2, 
    0x344, 0x345, 0x3, 0x2, 0x2, 0x2, 0x345, 0x346, 0x3, 0x2, 0x2, 0x2, 
    0x346, 0x348, 0x5, 0x5c, 0x2f, 0x2, 0x347, 0x344, 0x3, 0x2, 0x2, 0x2, 
    0x348, 0x34b, 0x3, 0x2, 0x2, 0x2, 0x349, 0x347, 0x3, 0x2, 0x2, 0x2, 
    0x349, 0x34a, 0x3, 0x2, 0x2, 0x2, 0x34a, 0x5b, 0x3, 0x2, 0x2, 0x2, 0x34b, 
    0x349, 0x3, 0x2, 0x2, 0x2, 0x34c, 0x34e, 0x7, 0xc, 0x2, 0x2, 0x34d, 
    0x34f, 0x7, 0x7f, 0x2, 0x2, 0x34e, 0x34d, 0x3, 0x2, 0x2, 0x2, 0x34e, 
    0x34f, 0x3, 0x2, 0x2, 0x2, 0x34f, 0x350, 0x3, 0x2, 0x2, 0x2, 0x350, 
    0x351, 0x5, 0x60, 0x31, 0x2, 0x351, 0x5d, 0x3, 0x2, 0x2, 0x2, 0x352, 
    0x354, 0x7, 0x7, 0x2, 0x2, 0x353, 0x355, 0x7, 0x7f, 0x2, 0x2, 0x354, 
    0x353, 0x3, 0x2, 0x2, 0x2, 0x354, 0x355, 0x3, 0x2, 0x2, 0x2, 0x355, 
    0x35a, 0x3, 0x2, 0x2, 0x2, 0x356, 0x358, 0x5, 0xb8, 0x5d, 0x2, 0x357, 
    0x359, 0x7, 0x7f, 0x2, 0x2, 0x358, 0x357, 0x3, 0x2, 0x2, 0x2, 0x358, 
    0x359, 0x3, 0x2, 0x2, 0x2, 0x359, 0x35b, 0x3, 0x2, 0x2, 0x2, 0x35a, 
    0x356, 0x3, 0x2, 0x2, 0x2, 0x35a, 0x35b, 0x3, 0x2, 0x2, 0x2, 0x35b, 
    0x366, 0x3, 0x2, 0x2, 0x2, 0x35c, 0x35e, 0x7, 0xe, 0x2, 0x2, 0x35d, 
    0x35f, 0x7, 0x7f, 0x2, 0x2, 0x35e, 0x35d, 0x3, 0x2, 0x2, 0x2, 0x35e, 
    0x35f, 0x3, 0x2, 0x2, 0x2, 0x35f, 0x364, 0x3, 0x2, 0x2, 0x2, 0x360, 
    0x362, 0x5, 0xb8, 0x5d, 0x2, 0x361, 0x363, 0x7, 0x7f, 0x2, 0x2, 0x362, 
    0x361, 0x3, 0x2, 0x2, 0x2, 0x362, 0x363, 0x3, 0x2, 0x2, 0x2, 0x363, 
    0x365, 0x3, 0x2, 0x2, 0x2, 0x364, 0x360, 0x3, 0x2, 0x2, 0x2, 0x364, 
    0x365, 0x3, 0x2, 0x2, 0x2, 0x365, 0x367, 0x3, 0x2, 0x2, 0x2, 0x366, 
    0x35c, 0x3, 0x2, 0x2, 0x2, 0x366, 0x367, 0x3, 0x2, 0x2, 0x2, 0x367, 
    0x5f, 0x3, 0x2, 0x2, 0x2, 0x368, 0x369, 0x5, 0xbc, 0x5f, 0x2, 0x369, 
    0x61, 0x3, 0x2, 0x2, 0x2, 0x36a, 0x36b, 0x5, 0xbc, 0x5f, 0x2, 0x36b, 
    0x63, 0x3, 0x2, 0x2, 0x2, 0x36c, 0x36d, 0x5, 0x66, 0x34, 0x2, 0x36d, 
    0x65, 0x3, 0x2, 0x2, 0x2, 0x36e, 0x375, 0x5, 0x68, 0x35, 0x2, 0x36f, 
    0x370, 0x7, 0x7f, 0x2, 0x2, 0x370, 0x371, 0x7, 0x4b, 0x2, 0x2, 0x371, 
    0x372, 0x7, 0x7f, 0x2, 0x2, 0x372, 0x374, 0x5, 0x68, 0x35, 0x2, 0x373, 
    0x36f, 0x3, 0x2, 0x2, 0x2, 0x374, 0x377, 0x3, 0x2, 0x2, 0x2, 0x375, 
    0x373, 0x3, 0x2, 0x2, 0x2, 0x375, 0x376, 0x3, 0x2, 0x2, 0x2, 0x376, 
    0x67, 0x3, 0x2, 0x2, 0x2, 0x377, 0x375, 0x3, 0x2, 0x2, 0x2, 0x378, 0x37f, 
    0x5, 0x6a, 0x36, 0x2, 0x379, 0x37a, 0x7, 0x7f, 0x2, 0x2, 0x37a, 0x37b, 
    0x7, 0x4c, 0x2, 0x2, 0x37b, 0x37c, 0x7, 0x7f, 0x2, 0x2, 0x37c, 0x37e, 
    0x5, 0x6a, 0x36, 0x2, 0x37d, 0x379, 0x3, 0x2, 0x2, 0x2, 0x37e, 0x381, 
    0x3, 0x2, 0x2, 0x2, 0x37f, 0x37d, 0x3, 0x2, 0x2, 0x2, 0x37f, 0x380, 
    0x3, 0x2, 0x2, 0x2, 0x380, 0x69, 0x3, 0x2, 0x2, 0x2, 0x381, 0x37f, 0x3, 
    0x2, 0x2, 0x2, 0x382, 0x389, 0x5, 0x6c, 0x37, 0x2, 0x383, 0x384, 0x7, 
    0x7f, 0x2, 0x2, 0x384, 0x385, 0x7, 0x4d, 0x2, 0x2, 0x385, 0x386, 0x7, 
    0x7f, 0x2, 0x2, 0x386, 0x388, 0x5, 0x6c, 0x37, 0x2, 0x387, 0x383, 0x3, 
    0x2, 0x2, 0x2, 0x388, 0x38b, 0x3, 0x2, 0x2, 0x2, 0x389, 0x387, 0x3, 
    0x2, 0x2, 0x2, 0x389, 0x38a, 0x3, 0x2, 0x2, 0x2, 0x38a, 0x6b, 0x3, 0x2, 
    0x2, 0x2, 0x38b, 0x389, 0x3, 0x2, 0x2, 0x2, 0x38c, 0x38e, 0x7, 0x4e, 
    0x2, 0x2, 0x38d, 0x38f, 0x7, 0x7f, 0x2, 0x2, 0x38e, 0x38d, 0x3, 0x2, 
    0x2, 0x2, 0x38e, 0x38f, 0x3, 0x2, 0x2, 0x2, 0x38f, 0x391, 0x3, 0x2, 
    0x2, 0x2, 0x390, 0x38c, 0x3, 0x2, 0x2, 0x2, 0x391, 0x394, 0x3, 0x2, 
    0x2, 0x2, 0x392, 0x390, 0x3, 0x2, 0x2, 0x2, 0x392, 0x393, 0x3, 0x2, 
    0x2, 0x2, 0x393, 0x395, 0x3, 0x2, 0x2, 0x2, 0x394, 0x392, 0x3, 0x2, 
    0x2, 0x2, 0x395, 0x396, 0x5, 0x6e, 0x38, 0x2, 0x396, 0x6d, 0x3, 0x2, 
    0x2, 0x2, 0x397, 0x39e, 0x5, 0x70, 0x39, 0x2, 0x398, 0x39a, 0x7, 0x7f, 
    0x2, 0x2, 0x399, 0x398, 0x3, 0x2, 0x2, 0x2, 0x399, 0x39a, 0x3, 0x2, 
    0x2, 0x2, 0x39a, 0x39b, 0x3, 0x2, 0x2, 0x2, 0x39b, 0x39d, 0x5, 0x8a, 
    0x46, 0x2, 0x39c, 0x399, 0x3, 0x2, 0x2, 0x2, 0x39d, 0x3a0, 0x3, 0x2, 
    0x2, 0x2, 0x39e, 0x39c, 0x3, 0x2, 0x2, 0x2, 0x39e, 0x39f, 0x3, 0x2, 
    0x2, 0x2, 0x39f, 0x6f, 0x3, 0x2, 0x2, 0x2, 0x3a0, 0x39e, 0x3, 0x2, 0x2, 
    0x2, 0x3a1, 0x3b4, 0x5, 0x72, 0x3a, 0x2, 0x3a2, 0x3a4, 0x7, 0x7f, 0x2, 
    0x2, 0x3a3, 0x3a2, 0x3, 0x2, 0x2, 0x2, 0x3a3, 0x3a4, 0x3, 0x2, 0x2, 
    0x2, 0x3a4, 0x3a5, 0x3, 0x2, 0x2, 0x2, 0x3a5, 0x3a7, 0x7, 0xf, 0x2, 
    0x2, 0x3a6, 0x3a8, 0x7, 0x7f, 0x2, 0x2, 0x3a7, 0x3a6, 0x3, 0x2, 0x2, 
    0x2, 0x3a7, 0x3a8, 0x3, 0x2, 0x2, 0x2, 0x3a8, 0x3a9, 0x3, 0x2, 0x2, 
    0x2, 0x3a9, 0x3b3, 0x5, 0x72, 0x3a, 0x2, 0x3aa, 0x3ac, 0x7, 0x7f, 0x2, 
    0x2, 0x3ab, 0x3aa, 0x3, 0x2, 0x2, 0x2, 0x3ab, 0x3ac, 0x3, 0x2, 0x2, 
    0x2, 0x3ac, 0x3ad, 0x3, 0x2, 0x2, 0x2, 0x3ad, 0x3af, 0x7, 0x10, 0x2, 
    0x2, 0x3ae, 0x3b0, 0x7, 0x7f, 0x2, 0x2, 0x3af, 0x3ae, 0x3, 0x2, 0x2, 
    0x2, 0x3af, 0x3b0, 0x3, 0x2, 0x2, 0x2, 0x3b0, 0x3b1, 0x3, 0x2, 0x2, 
    0x2, 0x3b1, 0x3b3, 0x5, 0x72, 0x3a, 0x2, 0x3b2, 0x3a3, 0x3, 0x2, 0x2, 
    0x2, 0x3b2, 0x3ab, 0x3, 0x2, 0x2, 0x2, 0x3b3, 0x3b6, 0x3, 0x2, 0x2, 
    0x2, 0x3b4, 0x3b2, 0x3, 0x2, 0x2, 0x2, 0x3b4, 0x3b5, 0x3, 0x2, 0x2, 
    0x2, 0x3b5, 0x71, 0x3, 0x2, 0x2, 0x2, 0x3b6, 0x3b4, 0x3, 0x2, 0x2, 0x2, 
    0x3b7, 0x3d2, 0x5, 0x74, 0x3b, 0x2, 0x3b8, 0x3ba, 0x7, 0x7f, 0x2, 0x2, 
    0x3b9, 0x3b8, 0x3, 0x2, 0x2, 0x2, 0x3b9, 0x3ba, 0x3, 0x2, 0x2, 0x2, 
    0x3ba, 0x3bb, 0x3, 0x2, 0x2, 0x2, 0x3bb, 0x3bd, 0x7, 0x7, 0x2, 0x2, 
    0x3bc, 0x3be, 0x7, 0x7f, 0x2, 0x2, 0x3bd, 0x3bc, 0x3, 0x2, 0x2, 0x2, 
    0x3bd, 0x3be, 0x3, 0x2, 0x2, 0x2, 0x3be, 0x3bf, 0x3, 0x2, 0x2, 0x2, 
    0x3bf, 0x3d1, 0x5, 0x74, 0x3b, 0x2, 0x3c0, 0x3c2, 0x7, 0x7f, 0x2, 0x2, 
    0x3c1, 0x3c0, 0x3, 0x2, 0x2, 0x2, 0x3c1, 0x3c2, 0x3, 0x2, 0x2, 0x2, 
    0x3c2, 0x3c3, 0x3, 0x2, 0x2, 0x2, 0x3c3, 0x3c5, 0x7, 0x11, 0x2, 0x2, 
    0x3c4, 0x3c6, 0x7, 0x7f, 0x2, 0x2, 0x3c5, 0x3c4, 0x3, 0x2, 0x2, 0x2, 
    0x3c5, 0x3c6, 0x3, 0x2, 0x2, 0x2, 0x3c6, 0x3c7, 0x3, 0x2, 0x2, 0x2, 
    0x3c7, 0x3d1, 0x5, 0x74, 0x3b, 0x2, 0x3c8, 0x3ca, 0x7, 0x7f, 0x2, 0x2, 
    0x3c9, 0x3c8, 0x3, 0x2, 0x2, 0x2, 0x3c9, 0x3ca, 0x3, 0x2, 0x2, 0x2, 
    0x3ca, 0x3cb, 0x3, 0x2, 0x2, 0x2, 0x3cb, 0x3cd, 0x7, 0x12, 0x2, 0x2, 
    0x3cc, 0x3ce, 0x7, 0x7f, 0x2, 0x2, 0x3cd, 0x3cc, 0x3, 0x2, 0x2, 0x2, 
    0x3cd, 0x3ce, 0x3, 0x2, 0x2, 0x2, 0x3ce, 0x3cf, 0x3, 0x2, 0x2, 0x2, 
    0x3cf, 0x3d1, 0x5, 0x74, 0x3b, 0x2, 0x3d0, 0x3b9, 0x3, 0x2, 0x2, 0x2, 
    0x3d0, 0x3c1, 0x3, 0x2, 0x2, 0x2, 0x3d0, 0x3c9, 0x3, 0x2, 0x2, 0x2, 
    0x3d1, 0x3d4, 0x3, 0x2, 0x2, 0x2, 0x3d2, 0x3d0, 0x3, 0x2, 0x2, 0x2, 
    0x3d2, 0x3d3, 0x3, 0x2, 0x2, 0x2, 0x3d3, 0x73, 0x3, 0x2, 0x2, 0x2, 0x3d4, 
    0x3d2, 0x3, 0x2, 0x2, 0x2, 0x3d5, 0x3e0, 0x5, 0x76, 0x3c, 0x2, 0x3d6, 
    0x3d8, 0x7, 0x7f, 0x2, 0x2, 0x3d7, 0x3d6, 0x3, 0x2, 0x2, 0x2, 0x3d7, 
    0x3d8, 0x3, 0x2, 0x2, 0x2, 0x3d8, 0x3d9, 0x3, 0x2, 0x2, 0x2, 0x3d9, 
    0x3db, 0x7, 0x13, 0x2, 0x2, 0x3da, 0x3dc, 0x7, 0x7f, 0x2, 0x2, 0x3db, 
    0x3da, 0x3, 0x2, 0x2, 0x2, 0x3db, 0x3dc, 0x3, 0x2, 0x2, 0x2, 0x3dc, 
    0x3dd, 0x3, 0x2, 0x2, 0x2, 0x3dd, 0x3df, 0x5, 0x76, 0x3c, 0x2, 0x3de, 
    0x3d7, 0x3, 0x2, 0x2, 0x2, 0x3df, 0x3e2, 0x3, 0x2, 0x2, 0x2, 0x3e0, 
    0x3de, 0x3, 0x2, 0x2, 0x2, 0x3e0, 0x3e1, 0x3, 0x2, 0x2, 0x2, 0x3e1, 
    0x75, 0x3, 0x2, 0x2, 0x2, 0x3e2, 0x3e0, 0x3, 0x2, 0x2, 0x2, 0x3e3, 0x3e5, 
    0x9, 0x3, 0x2, 0x2, 0x3e4, 0x3e6, 0x7, 0x7f, 0x2, 0x2, 0x3e5, 0x3e4, 
    0x3, 0x2, 0x2, 0x2, 0x3e5, 0x3e6, 0x3, 0x2, 0x2, 0x2, 0x3e6, 0x3e8, 
    0x3, 0x2, 0x2, 0x2, 0x3e7, 0x3e3, 0x3, 0x2, 0x2, 0x2, 0x3e8, 0x3eb, 
    0x3, 0x2, 0x2, 0x2, 0x3e9, 0x3e7, 0x3, 0x2, 0x2, 0x2, 0x3e9, 0x3ea, 
    0x3, 0x2, 0x2, 0x2, 0x3ea, 0x3ec, 0x3, 0x2, 0x2, 0x2, 0x3eb, 0x3e9, 
    0x3, 0x2, 0x2, 0x2, 0x3ec, 0x3ed, 0x5, 0x78, 0x3d, 0x2, 0x3ed, 0x77, 
    0x3, 0x2, 0x2, 0x2, 0x3ee, 0x3f4, 0x5, 0x80, 0x41, 0x2, 0x3ef, 0x3f3, 
    0x5, 0x7c, 0x3f, 0x2, 0x3f0, 0x3f3, 0x5, 0x7a, 0x3e, 0x2, 0x3f1, 0x3f3, 
    0x5, 0x7e, 0x40, 0x2, 0x3f2, 0x3ef, 0x3, 0x2, 0x2, 0x2, 0x3f2, 0x3f0, 
    0x3, 0x2, 0x2, 0x2, 0x3f2, 0x3f1, 0x3, 0x2, 0x2, 0x2, 0x3f3, 0x3f6, 
    0x3, 0x2, 0x2, 0x2, 0x3f4, 0x3f2, 0x3, 0x2, 0x2, 0x2, 0x3f4, 0x3f5, 
    0x3, 0x2, 0x2, 0x2, 0x3f5, 0x79, 0x3, 0x2, 0x2, 0x2, 0x3f6, 0x3f4, 0x3, 
    0x2, 0x2, 0x2, 0x3f7, 0x3f8, 0x7, 0x7f, 0x2, 0x2, 0x3f8, 0x3fa, 0x7, 
    0x4f, 0x2, 0x2, 0x3f9, 0x3fb, 0x7, 0x7f, 0x2, 0x2, 0x3fa, 0x3f9, 0x3, 
    0x2, 0x2, 0x2, 0x3fa, 0x3fb, 0x3, 0x2, 0x2, 0x2, 0x3fb, 0x3fc, 0x3, 
    0x2, 0x2, 0x2, 0x3fc, 0x411, 0x5, 0x80, 0x41, 0x2, 0x3fd, 0x3ff, 0x7, 
    0x7f, 0x2, 0x2, 0x3fe, 0x3fd, 0x3, 0x2, 0x2, 0x2, 0x3fe, 0x3ff, 0x3, 
    0x2, 0x2, 0x2, 0x3ff, 0x400, 0x3, 0x2, 0x2, 0x2, 0x400, 0x401, 0x7, 
    0xa, 0x2, 0x2, 0x401, 0x402, 0x5, 0x64, 0x33, 0x2, 0x402, 0x403, 0x7, 
    0xb, 0x2, 0x2, 0x403, 0x411, 0x3, 0x2, 0x2, 0x2, 0x404, 0x406, 0x7, 
    0x7f, 0x2, 0x2, 0x405, 0x404, 0x3, 0x2, 0x2, 0x2, 0x405, 0x406, 0x3, 
    0x2, 0x2, 0x2, 0x406, 0x407, 0x3, 0x2, 0x2, 0x2, 0x407, 0x409, 0x7, 
    0xa, 0x2, 0x2, 0x408, 0x40a, 0x5, 0x64, 0x33, 0x2, 0x409, 0x408, 0x3, 
    0x2, 0x2, 0x2, 0x409, 0x40a, 0x3, 0x2, 0x2, 0x2, 0x40a, 0x40b, 0x3, 
    0x2, 0x2, 0x2, 0x40b, 0x40d, 0x7, 0xe, 0x2, 0x2, 0x40c, 0x40e, 0x5, 
    0x64, 0x33, 0x2, 0x40d, 0x40c, 0x3, 0x2, 0x2, 0x2, 0x40d, 0x40e, 0x3, 
    0x2, 0x2, 0x2, 0x40e, 0x40f, 0x3, 0x2, 0x2, 0x2, 0x40f, 0x411, 0x7, 
    0xb, 0x2, 0x2, 0x410, 0x3f7, 0x3, 0x2, 0x2, 0x2, 0x410, 0x3fe, 0x3, 
    0x2, 0x2, 0x2, 0x410, 0x405, 0x3, 0x2, 0x2, 0x2, 0x411, 0x7b, 0x3, 0x2, 
    0x2, 0x2, 0x412, 0x413, 0x7, 0x7f, 0x2, 0x2, 0x413, 0x414, 0x7, 0x50, 
    0x2, 0x2, 0x414, 0x415, 0x7, 0x7f, 0x2, 0x2, 0x415, 0x41d, 0x7, 0x3f, 
    0x2, 0x2, 0x416, 0x417, 0x7, 0x7f, 0x2, 0x2, 0x417, 0x418, 0x7, 0x51, 
    0x2, 0x2, 0x418, 0x419, 0x7, 0x7f, 0x2, 0x2, 0x419, 0x41d, 0x7, 0x3f, 
    0x2, 0x2, 0x41a, 0x41b, 0x7, 0x7f, 0x2, 0x2, 0x41b, 0x41d, 0x7, 0x52, 
    0x2, 0x2, 0x41c, 0x412, 0x3, 0x2, 0x2, 0x2, 0x41c, 0x416, 0x3, 0x2, 
    0x2, 0x2, 0x41c, 0x41a, 0x3, 0x2, 0x2, 0x2, 0x41d, 0x41f, 0x3, 0x2, 
    0x2, 0x2, 0x41e, 0x420, 0x7, 0x7f, 0x2, 0x2, 0x41f, 0x41e, 0x3, 0x2, 
    0x2, 0x2, 0x41f, 0x420, 0x3, 0x2, 0x2, 0x2, 0x420, 0x421, 0x3, 0x2, 
    0x2, 0x2, 0x421, 0x422, 0x5, 0x80, 0x41, 0x2, 0x422, 0x7d, 0x3, 0x2, 
    0x2, 0x2, 0x423, 0x424, 0x7, 0x7f, 0x2, 0x2, 0x424, 0x425, 0x7, 0x53, 
    0x2, 0x2, 0x425, 0x426, 0x7, 0x7f, 0x2, 0x2, 0x426, 0x42e, 0x7, 0x54, 
    0x2, 0x2, 0x427, 0x428, 0x7, 0x7f, 0x2, 0x2, 0x428, 0x429, 0x7, 0x53, 
    0x2, 0x2, 0x429, 0x42a, 0x7, 0x7f, 0x2, 0x2, 0x42a, 0x42b, 0x7, 0x4e, 
    0x2, 0x2, 0x42b, 0x42c, 0x7, 0x7f, 0x2, 0x2, 0x42c, 0x42e, 0x7, 0x54, 
    0x2, 0x2, 0x42d, 0x423, 0x3, 0x2, 0x2, 0x2, 0x42d, 0x427, 0x3, 0x2, 
    0x2, 0x2, 0x42e, 0x7f, 0x3, 0x2, 0x2, 0x2, 0x42f, 0x436, 0x5, 0x82, 
    0x42, 0x2, 0x430, 0x432, 0x7, 0x7f, 0x2, 0x2, 0x431, 0x430, 0x3, 0x2, 
    0x2, 0x2, 0x431, 0x432, 0x3, 0x2, 0x2, 0x2, 0x432, 0x433, 0x3, 0x2, 
    0x2, 0x2, 0x433, 0x435, 0x5, 0xa6, 0x54, 0x2, 0x434, 0x431, 0x3, 0x2, 
    0x2, 0x2, 0x435, 0x438, 0x3, 0x2, 0x2, 0x2, 0x436, 0x434, 0x3, 0x2, 
    0x2, 0x2, 0x436, 0x437, 0x3, 0x2, 0x2, 0x2, 0x437, 0x43d, 0x3, 0x2, 
    0x2, 0x2, 0x438, 0x436, 0x3, 0x2, 0x2, 0x2, 0x439, 0x43b, 0x7, 0x7f, 
    0x2, 0x2, 0x43a, 0x439, 0x3, 0x2, 0x2, 0x2, 0x43a, 0x43b, 0x3, 0x2, 
    0x2, 0x2, 0x43b, 0x43c, 0x3, 0x2, 0x2, 0x2, 0x43c, 0x43e, 0x5, 0x5a, 
    0x2e, 0x2, 0x43d, 0x43a, 0x3, 0x2, 0x2, 0x2, 0x43d, 0x43e, 0x3, 0x2, 
    0x2, 0x2, 0x43e, 0x81, 0x3, 0x2, 0x2, 0x2, 0x43f, 0x48e, 0x5, 0x84, 
    0x43, 0x2, 0x440, 0x48e, 0x5, 0xb2, 0x5a, 0x2, 0x441, 0x48e, 0x5, 0xa8, 
    0x55, 0x2, 0x442, 0x444, 0x7, 0x55, 0x2, 0x2, 0x443, 0x445, 0x7, 0x7f, 
    0x2, 0x2, 0x444, 0x443, 0x3, 0x2, 0x2, 0x2, 0x444, 0x445, 0x3, 0x2, 
    0x2, 0x2, 0x445, 0x446, 0x3, 0x2, 0x2, 0x2, 0x446, 0x448, 0x7, 0x8, 
    0x2, 0x2, 0x447, 0x449, 0x7, 0x7f, 0x2, 0x2, 0x448, 0x447, 0x3, 0x2, 
    0x2, 0x2, 0x448, 0x449, 0x3, 0x2, 0x2, 0x2, 0x449, 0x44a, 0x3, 0x2, 
    0x2, 0x2, 0x44a, 0x44c, 0x7, 0x7, 0x2, 0x2, 0x44b, 0x44d, 0x7, 0x7f, 
    0x2, 0x2, 0x44c, 0x44b, 0x3, 0x2, 0x2, 0x2, 0x44c, 0x44d, 0x3, 0x2, 
    0x2, 0x2, 0x44d, 0x44e, 0x3, 0x2, 0x2, 0x2, 0x44e, 0x48e, 0x7, 0x9, 
    0x2, 0x2, 0x44f, 0x48e, 0x5, 0xa2, 0x52, 0x2, 0x450, 0x48e, 0x5, 0xa4, 
    0x53, 0x2, 0x451, 0x453, 0x7, 0x31, 0x2, 0x2, 0x452, 0x454, 0x7, 0x7f, 
    0x2, 0x2, 0x453, 0x452, 0x3, 0x2, 0x2, 0x2, 0x453, 0x454, 0x3, 0x2, 
    0x2, 0x2, 0x454, 0x455, 0x3, 0x2, 0x2, 0x2, 0x455, 0x457, 0x7, 0x8, 
    0x2, 0x2, 0x456, 0x458, 0x7, 0x7f, 0x2, 0x2, 0x457, 0x456, 0x3, 0x2, 
    0x2, 0x2, 0x457, 0x458, 0x3, 0x2, 0x2, 0x2, 0x458, 0x459, 0x3, 0x2, 
    0x2, 0x2, 0x459, 0x45b, 0x5, 0x90, 0x49, 0x2, 0x45a, 0x45c, 0x7, 0x7f, 
    0x2, 0x2, 0x45b, 0x45a, 0x3, 0x2, 0x2, 0x2, 0x45b, 0x45c, 0x3, 0x2, 
    0x2, 0x2, 0x45c, 0x45d, 0x3, 0x2, 0x2, 0x2, 0x45d, 0x45e, 0x7, 0x9, 
    0x2, 0x2, 0x45e, 0x48e, 0x3, 0x2, 0x2, 0x2, 0x45f, 0x461, 0x7, 0x56, 
    0x2, 0x2, 0x460, 0x462, 0x7, 0x7f, 0x2, 0x2, 0x461, 0x460, 0x3, 0x2, 
    0x2, 0x2, 0x461, 0x462, 0x3, 0x2, 0x2, 0x2, 0x462, 0x463, 0x3, 0x2, 
    0x2, 0x2, 0x463, 0x465, 0x7, 0x8, 0x2, 0x2, 0x464, 0x466, 0x7, 0x7f, 
    0x2, 0x2, 0x465, 0x464, 0x3, 0x2, 0x2, 0x2, 0x465, 0x466, 0x3, 0x2, 
    0x2, 0x2, 0x466, 0x467, 0x3, 0x2, 0x2, 0x2, 0x467, 0x469, 0x5, 0x90, 
    0x49, 0x2, 0x468, 0x46a, 0x7, 0x7f, 0x2, 0x2, 0x469, 0x468, 0x3, 0x2, 
    0x2, 0x2, 0x469, 0x46a, 0x3, 0x2, 0x2, 0x2, 0x46a, 0x46b, 0x3, 0x2, 
    0x2, 0x2, 0x46b, 0x46c, 0x7, 0x9, 0x2, 0x2, 0x46c, 0x48e, 0x3, 0x2, 
    0x2, 0x2, 0x46d, 0x46f, 0x7, 0x57, 0x2, 0x2, 0x46e, 0x470, 0x7, 0x7f, 
    0x2, 0x2, 0x46f, 0x46e, 0x3, 0x2, 0x2, 0x2, 0x46f, 0x470, 0x3, 0x2, 
    0x2, 0x2, 0x470, 0x471, 0x3, 0x2, 0x2, 0x2, 0x471, 0x473, 0x7, 0x8, 
    0x2, 0x2, 0x472, 0x474, 0x7, 0x7f, 0x2, 0x2, 0x473, 0x472, 0x3, 0x2, 
    0x2, 0x2, 0x473, 0x474, 0x3, 0x2, 0x2, 0x2, 0x474, 0x475, 0x3, 0x2, 
    0x2, 0x2, 0x475, 0x477, 0x5, 0x90, 0x49, 0x2, 0x476, 0x478, 0x7, 0x7f, 
    0x2, 0x2, 0x477, 0x476, 0x3, 0x2, 0x2, 0x2, 0x477, 0x478, 0x3, 0x2, 
    0x2, 0x2, 0x478, 0x479, 0x3, 0x2, 0x2, 0x2, 0x479, 0x47a, 0x7, 0x9, 
    0x2, 0x2, 0x47a, 0x48e, 0x3, 0x2, 0x2, 0x2, 0x47b, 0x47d, 0x7, 0x58, 
    0x2, 0x2, 0x47c, 0x47e, 0x7, 0x7f, 0x2, 0x2, 0x47d, 0x47c, 0x3, 0x2, 
    0x2, 0x2, 0x47d, 0x47e, 0x3, 0x2, 0x2, 0x2, 0x47e, 0x47f, 0x3, 0x2, 
    0x2, 0x2, 0x47f, 0x481, 0x7, 0x8, 0x2, 0x2, 0x480, 0x482, 0x7, 0x7f, 
    0x2, 0x2, 0x481, 0x480, 0x3, 0x2, 0x2, 0x2, 0x481, 0x482, 0x3, 0x2, 
    0x2, 0x2, 0x482, 0x483, 0x3, 0x2, 0x2, 0x2, 0x483, 0x485, 0x5, 0x90, 
    0x49, 0x2, 0x484, 0x486, 0x7, 0x7f, 0x2, 0x2, 0x485, 0x484, 0x3, 0x2, 
    0x2, 0x2, 0x485, 0x486, 0x3, 0x2, 0x2, 0x2, 0x486, 0x487, 0x3, 0x2, 
    0x2, 0x2, 0x487, 0x488, 0x7, 0x9, 0x2, 0x2, 0x488, 0x48e, 0x3, 0x2, 
    0x2, 0x2, 0x489, 0x48e, 0x5, 0x8e, 0x48, 0x2, 0x48a, 0x48e, 0x5, 0x8c, 
    0x47, 0x2, 0x48b, 0x48e, 0x5, 0x94, 0x4b, 0x2, 0x48c, 0x48e, 0x5, 0xac, 
    0x57, 0x2, 0x48d, 0x43f, 0x3, 0x2, 0x2, 0x2, 0x48d, 0x440, 0x3, 0x2, 
    0x2, 0x2, 0x48d, 0x441, 0x3, 0x2, 0x2, 0x2, 0x48d, 0x442, 0x3, 0x2, 
    0x2, 0x2, 0x48d, 0x44f, 0x3, 0x2, 0x2, 0x2, 0x48d, 0x450, 0x3, 0x2, 
    0x2, 0x2, 0x48d, 0x451, 0x3, 0x2, 0x2, 0x2, 0x48d, 0x45f, 0x3, 0x2, 
    0x2, 0x2, 0x48d, 0x46d, 0x3, 0x2, 0x2, 0x2, 0x48d, 0x47b, 0x3, 0x2, 
    0x2, 0x2, 0x48d, 0x489, 0x3, 0x2, 0x2, 0x2, 0x48d, 0x48a, 0x3, 0x2, 
    0x2, 0x2, 0x48d, 0x48b, 0x3, 0x2, 0x2, 0x2, 0x48d, 0x48c, 0x3, 0x2, 
    0x2, 0x2, 0x48e, 0x83, 0x3, 0x2, 0x2, 0x2, 0x48f, 0x496, 0x5, 0xae, 
    0x58, 0x2, 0x490, 0x496, 0x7, 0x61, 0x2, 0x2, 0x491, 0x496, 0x5, 0x86, 
    0x44, 0x2, 0x492, 0x496, 0x7, 0x54, 0x2, 0x2, 0x493, 0x496, 0x5, 0xb0, 
    0x59, 0x2, 0x494, 0x496, 0x5, 0x88, 0x45, 0x2, 0x495, 0x48f, 0x3, 0x2, 
    0x2, 0x2, 0x495, 0x490, 0x3, 0x2, 0x2, 0x2, 0x495, 0x491, 0x3, 0x2, 
    0x2, 0x2, 0x495, 0x492, 0x3, 0x2, 0x2, 0x2, 0x495, 0x493, 0x3, 0x2, 
    0x2, 0x2, 0x495, 0x494, 0x3, 0x2, 0x2, 0x2, 0x496, 0x85, 0x3, 0x2, 0x2, 
    0x2, 0x497, 0x498, 0x9, 0x4, 0x2, 0x2, 0x498, 0x87, 0x3, 0x2, 0x2, 0x2, 
    0x499, 0x49b, 0x7, 0xa, 0x2, 0x2, 0x49a, 0x49c, 0x7, 0x7f, 0x2, 0x2, 
    0x49b, 0x49a, 0x3, 0x2, 0x2, 0x2, 0x49b, 0x49c, 0x3, 0x2, 0x2, 0x2, 
    0x49c, 0x4ae, 0x3, 0x2, 0x2, 0x2, 0x49d, 0x49f, 0x5, 0x64, 0x33, 0x2, 
    0x49e, 0x4a0, 0x7, 0x7f, 0x2, 0x2, 0x49f, 0x49e, 0x3, 0x2, 0x2, 0x2, 
    0x49f, 0x4a0, 0x3, 0x2, 0x2, 0x2, 0x4a0, 0x4ab, 0x3, 0x2, 0x2, 0x2, 
    0x4a1, 0x4a3, 0x7, 0x4, 0x2, 0x2, 0x4a2, 0x4a4, 0x7, 0x7f, 0x2, 0x2, 
    0x4a3, 0x4a2, 0x3, 0x2, 0x2, 0x2, 0x4a3, 0x4a4, 0x3, 0x2, 0x2, 0x2, 
    0x4a4, 0x4a5, 0x3, 0x2, 0x2, 0x2, 0x4a5, 0x4a7, 0x5, 0x64, 0x33, 0x2, 
    0x4a6, 0x4a8, 0x7, 0x7f, 0x2, 0x2, 0x4a7, 0x4a6, 0x3, 0x2, 0x2, 0x2, 
    0x4a7, 0x4a8, 0x3, 0x2, 0x2, 0x2, 0x4a8, 0x4aa, 0x3, 0x2, 0x2, 0x2, 
    0x4a9, 0x4a1, 0x3, 0x2, 0x2, 0x2, 0x4aa, 0x4ad, 0x3, 0x2, 0x2, 0x2, 
    0x4ab, 0x4a9, 0x3, 0x2, 0x2, 0x2, 0x4ab, 0x4ac, 0x3, 0x2, 0x2, 0x2, 
    0x4ac, 0x4af, 0x3, 0x2, 0x2, 0x2, 0x4ad, 0x4ab, 0x3, 0x2, 0x2, 0x2, 
    0x4ae, 0x49d, 0x3, 0x2, 0x2, 0x2, 0x4ae, 0x4af, 0x3, 0x2, 0x2, 0x2, 
    0x4af, 0x4b0, 0x3, 0x2, 0x2, 0x2, 0x4b0, 0x4b1, 0x7, 0xb, 0x2, 0x2, 
    0x4b1, 0x89, 0x3, 0x2, 0x2, 0x2, 0x4b2, 0x4b4, 0x7, 0x5, 0x2, 0x2, 0x4b3, 
    0x4b5, 0x7, 0x7f, 0x2, 0x2, 0x4b4, 0x4b3, 0x3, 0x2, 0x2, 0x2, 0x4b4, 
    0x4b5, 0x3, 0x2, 0x2, 0x2, 0x4b5, 0x4b6, 0x3, 0x2, 0x2, 0x2, 0x4b6, 
    0x4d1, 0x5, 0x70, 0x39, 0x2, 0x4b7, 0x4b9, 0x7, 0x14, 0x2, 0x2, 0x4b8, 
    0x4ba, 0x7, 0x7f, 0x2, 0x2, 0x4b9, 0x4b8, 0x3, 0x2, 0x2, 0x2, 0x4b9, 
    0x4ba, 0x3, 0x2, 0x2, 0x2, 0x4ba, 0x4bb, 0x3, 0x2, 0x2, 0x2, 0x4bb, 
    0x4d1, 0x5, 0x70, 0x39, 0x2, 0x4bc, 0x4be, 0x7, 0x15, 0x2, 0x2, 0x4bd, 
    0x4bf, 0x7, 0x7f, 0x2, 0x2, 0x4be, 0x4bd, 0x3, 0x2, 0x2, 0x2, 0x4be, 
    0x4bf, 0x3, 0x2, 0x2, 0x2, 0x4bf, 0x4c0, 0x3, 0x2, 0x2, 0x2, 0x4c0, 
    0x4d1, 0x5, 0x70, 0x39, 0x2, 0x4c1, 0x4c3, 0x7, 0x16, 0x2, 0x2, 0x4c2, 
    0x4c4, 0x7, 0x7f, 0x2, 0x2, 0x4c3, 0x4c2, 0x3, 0x2, 0x2, 0x2, 0x4c3, 
    0x4c4, 0x3, 0x2, 0x2, 0x2, 0x4c4, 0x4c5, 0x3, 0x2, 0x2, 0x2, 0x4c5, 
    0x4d1, 0x5, 0x70, 0x39, 0x2, 0x4c6, 0x4c8, 0x7, 0x17, 0x2, 0x2, 0x4c7, 
    0x4c9, 0x7, 0x7f, 0x2, 0x2, 0x4c8, 0x4c7, 0x3, 0x2, 0x2, 0x2, 0x4c8, 
    0x4c9, 0x3, 0x2, 0x2, 0x2, 0x4c9, 0x4ca, 0x3, 0x2, 0x2, 0x2, 0x4ca, 
    0x4d1, 0x5, 0x70, 0x39, 0x2, 0x4cb, 0x4cd, 0x7, 0x18, 0x2, 0x2, 0x4cc, 
    0x4ce, 0x7, 0x7f, 0x2, 0x2, 0x4cd, 0x4cc, 0x3, 0x2, 0x2, 0x2, 0x4cd, 
    0x4ce, 0x3, 0x2, 0x2, 0x2, 0x4ce, 0x4cf, 0x3, 0x2, 0x2, 0x2, 0x4cf, 
    0x4d1, 0x5, 0x70, 0x39, 0x2, 0x4d0, 0x4b2, 0x3, 0x2, 0x2, 0x2, 0x4d0, 
    0x4b7, 0x3, 0x2, 0x2, 0x2, 0x4d0, 0x4bc, 0x3, 0x2, 0x2, 0x2, 0x4d0, 
    0x4c1, 0x3, 0x2, 0x2, 0x2, 0x4d0, 0x4c6, 0x3, 0x2, 0x2, 0x2, 0x4d0, 
    0x4cb, 0x3, 0x2, 0x2, 0x2, 0x4d1, 0x8b, 0x3, 0x2, 0x2, 0x2, 0x4d2, 0x4d4, 
    0x7, 0x8, 0x2, 0x2, 0x4d3, 0x4d5, 0x7, 0x7f, 0x2, 0x2, 0x4d4, 0x4d3, 
    0x3, 0x2, 0x2, 0x2, 0x4d4, 0x4d5, 0x3, 0x2, 0x2, 0x2, 0x4d5, 0x4d6, 
    0x3, 0x2, 0x2, 0x2, 0x4d6, 0x4d8, 0x5, 0x64, 0x33, 0x2, 0x4d7, 0x4d9, 
    0x7, 0x7f, 0x2, 0x2, 0x4d8, 0x4d7, 0x3, 0x2, 0x2, 0x2, 0x4d8, 0x4d9, 
    0x3, 0x2, 0x2, 0x2, 0x4d9, 0x4da, 0x3, 0x2, 0x2, 0x2, 0x4da, 0x4db, 
    0x7, 0x9, 0x2, 0x2, 0x4db, 0x8d, 0x3, 0x2, 0x2, 0x2, 0x4dc, 0x4e1, 0x5, 
    0x4e, 0x28, 0x2, 0x4dd, 0x4df, 0x7, 0x7f, 0x2, 0x2, 0x4de, 0x4dd, 0x3, 
    0x2, 0x2, 0x2, 0x4de, 0x4df, 0x3, 0x2, 0x2, 0x2, 0x4df, 0x4e0, 0x3, 
    0x2, 0x2, 0x2, 0x4e0, 0x4e2, 0x5, 0x50, 0x29, 0x2, 0x4e1, 0x4de, 0x3, 
    0x2, 0x2, 0x2, 0x4e2, 0x4e3, 0x3, 0x2, 0x2, 0x2, 0x4e3, 0x4e1, 0x3, 
    0x2, 0x2, 0x2, 0x4e3, 0x4e4, 0x3, 0x2, 0x2, 0x2, 0x4e4, 0x8f, 0x3, 0x2, 
    0x2, 0x2, 0x4e5, 0x4ea, 0x5, 0x92, 0x4a, 0x2, 0x4e6, 0x4e8, 0x7, 0x7f, 
    0x2, 0x2, 0x4e7, 0x4e6, 0x3, 0x2, 0x2, 0x2, 0x4e7, 0x4e8, 0x3, 0x2, 
    0x2, 0x2, 0x4e8, 0x4e9, 0x3, 0x2, 0x2, 0x2, 0x4e9, 0x4eb, 0x5, 0x44, 
    0x23, 0x2, 0x4ea, 0x4e7, 0x3, 0x2, 0x2, 0x2, 0x4ea, 0x4eb, 0x3, 0x2, 
    0x2, 0x2, 0x4eb, 0x91, 0x3, 0x2, 0x2, 0x2, 0x4ec, 0x4ed, 0x5, 0xac, 
    0x57, 0x2, 0x4ed, 0x4ee, 0x7, 0x7f, 0x2, 0x2, 0x4ee, 0x4ef, 0x7, 0x4f, 
    0x2, 0x2, 0x4ef, 0x4f0, 0x7, 0x7f, 0x2, 0x2, 0x4f0, 0x4f1, 0x5, 0x64, 
    0x33, 0x2, 0x4f1, 0x93, 0x3, 0x2, 0x2, 0x2, 0x4f2, 0x4f4, 0x5, 0x96, 
    0x4c, 0x2, 0x4f3, 0x4f5, 0x7, 0x7f, 0x2, 0x2, 0x4f4, 0x4f3, 0x3, 0x2, 
    0x2, 0x2, 0x4f4, 0x4f5, 0x3, 0x2, 0x2, 0x2, 0x4f5, 0x4f6, 0x3, 0x2, 
    0x2, 0x2, 0x4f6, 0x4f8, 0x7, 0x8, 0x2, 0x2, 0x4f7, 0x4f9, 0x7, 0x7f, 
    0x2, 0x2, 0x4f8, 0x4f7, 0x3, 0x2, 0x2, 0x2, 0x4f8, 0x4f9, 0x3, 0x2, 
    0x2, 0x2, 0x4f9, 0x4fe, 0x3, 0x2, 0x2, 0x2, 0x4fa, 0x4fc, 0x7, 0x40, 
    0x2, 0x2, 0x4fb, 0x4fd, 0x7, 0x7f, 0x2, 0x2, 0x4fc, 0x4fb, 0x3, 0x2, 
    0x2, 0x2, 0x4fc, 0x4fd, 0x3, 0x2, 0x2, 0x2, 0x4fd, 0x4ff, 0x3, 0x2, 
    0x2, 0x2, 0x4fe, 0x4fa, 0x3, 0x2, 0x2, 0x2, 0x4fe, 0x4ff, 0x3, 0x2, 
    0x2, 0x2, 0x4ff, 0x511, 0x3, 0x2, 0x2, 0x2, 0x500, 0x502, 0x5, 0x64, 
    0x33, 0x2, 0x501, 0x503, 0x7, 0x7f, 0x2, 0x2, 0x502, 0x501, 0x3, 0x2, 
    0x2, 0x2, 0x502, 0x503, 0x3, 0x2, 0x2, 0x2, 0x503, 0x50e, 0x3, 0x2, 
    0x2, 0x2, 0x504, 0x506, 0x7, 0x4, 0x2, 0x2, 0x505, 0x507, 0x7, 0x7f, 
    0x2, 0x2, 0x506, 0x505, 0x3, 0x2, 0x2, 0x2, 0x506, 0x507, 0x3, 0x2, 
    0x2, 0x2, 0x507, 0x508, 0x3, 0x2, 0x2, 0x2, 0x508, 0x50a, 0x5, 0x64, 
    0x33, 0x2, 0x509, 0x50b, 0x7, 0x7f, 0x2, 0x2, 0x50a, 0x509, 0x3, 0x2, 
    0x2, 0x2, 0x50a, 0x50b, 0x3, 0x2, 0x2, 0x2, 0x50b, 0x50d, 0x3, 0x2, 
    0x2, 0x2, 0x50c, 0x504, 0x3, 0x2, 0x2, 0x2, 0x50d, 0x510, 0x3, 0x2, 
    0x2, 0x2, 0x50e, 0x50c, 0x3, 0x2, 0x2, 0x2, 0x50e, 0x50f, 0x3, 0x2, 
    0x2, 0x2, 0x50f, 0x512, 0x3, 0x2, 0x2, 0x2, 0x510, 0x50e, 0x3, 0x2, 
    0x2, 0x2, 0x511, 0x500, 0x3, 0x2, 0x2, 0x2, 0x511, 0x512, 0x3, 0x2, 
    0x2, 0x2, 0x512, 0x513, 0x3, 0x2, 0x2, 0x2, 0x513, 0x514, 0x7, 0x9, 
    0x2, 0x2, 0x514, 0x95, 0x3, 0x2, 0x2, 0x2, 0x515, 0x516, 0x5, 0xa0, 
    0x51, 0x2, 0x516, 0x517, 0x5, 0xc0, 0x61, 0x2, 0x517, 0x51a, 0x3, 0x2, 
    0x2, 0x2, 0x518, 0x51a, 0x7, 0x5b, 0x2, 0x2, 0x519, 0x515, 0x3, 0x2, 
    0x2, 0x2, 0x519, 0x518, 0x3, 0x2, 0x2, 0x2, 0x51a, 0x97, 0x3, 0x2, 0x2, 
    0x2, 0x51b, 0x51d, 0x5, 0x9e, 0x50, 0x2, 0x51c, 0x51e, 0x7, 0x7f, 0x2, 
    0x2, 0x51d, 0x51c, 0x3, 0x2, 0x2, 0x2, 0x51d, 0x51e, 0x3, 0x2, 0x2, 
    0x2, 0x51e, 0x51f, 0x3, 0x2, 0x2, 0x2, 0x51f, 0x521, 0x7, 0x8, 0x2, 
    0x2, 0x520, 0x522, 0x7, 0x7f, 0x2, 0x2, 0x521, 0x520, 0x3, 0x2, 0x2, 
    0x2, 0x521, 0x522, 0x3, 0x2, 0x2, 0x2, 0x522, 0x534, 0x3, 0x2, 0x2, 
    0x2, 0x523, 0x525, 0x5, 0x64, 0x33, 0x2, 0x524, 0x526, 0x7, 0x7f, 0x2, 
    0x2, 0x525, 0x524, 0x3, 0x2, 0x2, 0x2, 0x525, 0x526, 0x3, 0x2, 0x2, 
    0x2, 0x526, 0x531, 0x3, 0x2, 0x2, 0x2, 0x527, 0x529, 0x7, 0x4, 0x2, 
    0x2, 0x528, 0x52a, 0x7, 0x7f, 0x2, 0x2, 0x529, 0x528, 0x3, 0x2, 0x2, 
    0x2, 0x529, 0x52a, 0x3, 0x2, 0x2, 0x2, 0x52a, 0x52b, 0x3, 0x2, 0x2, 
    0x2, 0x52b, 0x52d, 0x5, 0x64, 0x33, 0x2, 0x52c, 0x52e, 0x7, 0x7f, 0x2, 
    0x2, 0x52d, 0x52c, 0x3, 0x2, 0x2, 0x2, 0x52d, 0x52e, 0x3, 0x2, 0x2, 
    0x2, 0x52e, 0x530, 0x3, 0x2, 0x2, 0x2, 0x52f, 0x527, 0x3, 0x2, 0x2, 
    0x2, 0x530, 0x533, 0x3, 0x2, 0x2, 0x2, 0x531, 0x52f, 0x3, 0x2, 0x2, 
    0x2, 0x531, 0x532, 0x3, 0x2, 0x2, 0x2, 0x532, 0x535, 0x3, 0x2, 0x2, 
    0x2, 0x533, 0x531, 0x3, 0x2, 0x2, 0x2, 0x534, 0x523, 0x3, 0x2, 0x2, 
    0x2, 0x534, 0x535, 0x3, 0x2, 0x2, 0x2, 0x535, 0x536, 0x3, 0x2, 0x2, 
    0x2, 0x536, 0x537, 0x7, 0x9, 0x2, 0x2, 0x537, 0x99, 0x3, 0x2, 0x2, 0x2, 
    0x538, 0x539, 0x5, 0x9e, 0x50, 0x2, 0x539, 0x9b, 0x3, 0x2, 0x2, 0x2, 
    0x53a, 0x53b, 0x5, 0xc0, 0x61, 0x2, 0x53b, 0x9d, 0x3, 0x2, 0x2, 0x2, 
    0x53c, 0x53d, 0x5, 0xa0, 0x51, 0x2, 0x53d, 0x53e, 0x5, 0xc0, 0x61, 0x2, 
    0x53e, 0x9f, 0x3, 0x2, 0x2, 0x2, 0x53f, 0x540, 0x5, 0xc0, 0x61, 0x2, 
    0x540, 0x541, 0x7, 0x19, 0x2, 0x2, 0x541, 0x543, 0x3, 0x2, 0x2, 0x2, 
    0x542, 0x53f, 0x3, 0x2, 0x2, 0x2, 0x543, 0x546, 0x3, 0x2, 0x2, 0x2, 
    0x544, 0x542, 0x3, 0x2, 0x2, 0x2, 0x544, 0x545, 0x3, 0x2, 0x2, 0x2, 
    0x545, 0xa1, 0x3, 0x2, 0x2, 0x2, 0x546, 0x544, 0x3, 0x2, 0x2, 0x2, 0x547, 
    0x549, 0x7, 0xa, 0x2, 0x2, 0x548, 0x54a, 0x7, 0x7f, 0x2, 0x2, 0x549, 
    0x548, 0x3, 0x2, 0x2, 0x2, 0x549, 0x54a, 0x3, 0x2, 0x2, 0x2, 0x54a, 
    0x54b, 0x3, 0x2, 0x2, 0x2, 0x54b, 0x554, 0x5, 0x90, 0x49, 0x2, 0x54c, 
    0x54e, 0x7, 0x7f, 0x2, 0x2, 0x54d, 0x54c, 0x3, 0x2, 0x2, 0x2, 0x54d, 
    0x54e, 0x3, 0x2, 0x2, 0x2, 0x54e, 0x54f, 0x3, 0x2, 0x2, 0x2, 0x54f, 
    0x551, 0x7, 0xd, 0x2, 0x2, 0x550, 0x552, 0x7, 0x7f, 0x2, 0x2, 0x551, 
    0x550, 0x3, 0x2, 0x2, 0x2, 0x551, 0x552, 0x3, 0x2, 0x2, 0x2, 0x552, 
    0x553, 0x3, 0x2, 0x2, 0x2, 0x553, 0x555, 0x5, 0x64, 0x33, 0x2, 0x554, 
    0x54d, 0x3, 0x2, 0x2, 0x2, 0x554, 0x555, 0x3, 0x2, 0x2, 0x2, 0x555, 
    0x557, 0x3, 0x2, 0x2, 0x2, 0x556, 0x558, 0x7, 0x7f, 0x2, 0x2, 0x557, 
    0x556, 0x3, 0x2, 0x2, 0x2, 0x557, 0x558, 0x3, 0x2, 0x2, 0x2, 0x558, 
    0x559, 0x3, 0x2, 0x2, 0x2, 0x559, 0x55a, 0x7, 0xb, 0x2, 0x2, 0x55a, 
    0xa3, 0x3, 0x2, 0x2, 0x2, 0x55b, 0x55d, 0x7, 0xa, 0x2, 0x2, 0x55c, 0x55e, 
    0x7, 0x7f, 0x2, 0x2, 0x55d, 0x55c, 0x3, 0x2, 0x2, 0x2, 0x55d, 0x55e, 
    0x3, 0x2, 0x2, 0x2, 0x55e, 0x567, 0x3, 0x2, 0x2, 0x2, 0x55f, 0x561, 
    0x5, 0xac, 0x57, 0x2, 0x560, 0x562, 0x7, 0x7f, 0x2, 0x2, 0x561, 0x560, 
    0x3, 0x2, 0x2, 0x2, 0x561, 0x562, 0x3, 0x2, 0x2, 0x2, 0x562, 0x563, 
    0x3, 0x2, 0x2, 0x2, 0x563, 0x565, 0x7, 0x5, 0x2, 0x2, 0x564, 0x566, 
    0x7, 0x7f, 0x2, 0x2, 0x565, 0x564, 0x3, 0x2, 0x2, 0x2, 0x565, 0x566, 
    0x3, 0x2, 0x2, 0x2, 0x566, 0x568, 0x3, 0x2, 0x2, 0x2, 0x567, 0x55f, 
    0x3, 0x2, 0x2, 0x2, 0x567, 0x568, 0x3, 0x2, 0x2, 0x2, 0x568, 0x569, 
    0x3, 0x2, 0x2, 0x2, 0x569, 0x56b, 0x5, 0x8e, 0x48, 0x2, 0x56a, 0x56c, 
    0x7, 0x7f, 0x2, 0x2, 0x56b, 0x56a, 0x3, 0x2, 0x2, 0x2, 0x56b, 0x56c, 
    0x3, 0x2, 0x2, 0x2, 0x56c, 0x575, 0x3, 0x2, 0x2, 0x2, 0x56d, 0x56f, 
    0x7, 0x4a, 0x2, 0x2, 0x56e, 0x570, 0x7, 0x7f, 0x2, 0x2, 0x56f, 0x56e, 
    0x3, 0x2, 0x2, 0x2, 0x56f, 0x570, 0x3, 0x2, 0x2, 0x2, 0x570, 0x571, 
    0x3, 0x2, 0x2, 0x2, 0x571, 0x573, 0x5, 0x64, 0x33, 0x2, 0x572, 0x574, 
    0x7, 0x7f, 0x2, 0x2, 0x573, 0x572, 0x3, 0x2, 0x2, 0x2, 0x573, 0x574, 
    0x3, 0x2, 0x2, 0x2, 0x574, 0x576, 0x3, 0x2, 0x2, 0x2, 0x575, 0x56d, 
    0x3, 0x2, 0x2, 0x2, 0x575, 0x576, 0x3, 0x2, 0x2, 0x2, 0x576, 0x577, 
    0x3, 0x2, 0x2, 0x2, 0x577, 0x579, 0x7, 0xd, 0x2, 0x2, 0x578, 0x57a, 
    0x7, 0x7f, 0x2, 0x2, 0x579, 0x578, 0x3, 0x2, 0x2, 0x2, 0x579, 0x57a, 
    0x3, 0x2, 0x2, 0x2, 0x57a, 0x57b, 0x3, 0x2, 0x2, 0x2, 0x57b, 0x57d, 
    0x5, 0x64, 0x33, 0x2, 0x57c, 0x57e, 0x7, 0x7f, 0x2, 0x2, 0x57d, 0x57c, 
    0x3, 0x2, 0x2, 0x2, 0x57d, 0x57e, 0x3, 0x2, 0x2, 0x2, 0x57e, 0x57f, 
    0x3, 0x2, 0x2, 0x2, 0x57f, 0x580, 0x7, 0xb, 0x2, 0x2, 0x580, 0xa5, 0x3, 
    0x2, 0x2, 0x2, 0x581, 0x583, 0x7, 0x19, 0x2, 0x2, 0x582, 0x584, 0x7, 
    0x7f, 0x2, 0x2, 0x583, 0x582, 0x3, 0x2, 0x2, 0x2, 0x583, 0x584, 0x3, 
    0x2, 0x2, 0x2, 0x584, 0x585, 0x3, 0x2, 0x2, 0x2, 0x585, 0x586, 0x5, 
    0xb6, 0x5c, 0x2, 0x586, 0xa7, 0x3, 0x2, 0x2, 0x2, 0x587, 0x58c, 0x7, 
    0x5c, 0x2, 0x2, 0x588, 0x58a, 0x7, 0x7f, 0x2, 0x2, 0x589, 0x588, 0x3, 
    0x2, 0x2, 0x2, 0x589, 0x58a, 0x3, 0x2, 0x2, 0x2, 0x58a, 0x58b, 0x3, 
    0x2, 0x2, 0x2, 0x58b, 0x58d, 0x5, 0xaa, 0x56, 0x2, 0x58c, 0x589, 0x3, 
    0x2, 0x2, 0x2, 0x58d, 0x58e, 0x3, 0x2, 0x2, 0x2, 0x58e, 0x58c, 0x3, 
    0x2, 0x2, 0x2, 0x58e, 0x58f, 0x3, 0x2, 0x2, 0x2, 0x58f, 0x59e, 0x3, 
    0x2, 0x2, 0x2, 0x590, 0x592, 0x7, 0x5c, 0x2, 0x2, 0x591, 0x593, 0x7, 
    0x7f, 0x2, 0x2, 0x592, 0x591, 0x3, 0x2, 0x2, 0x2, 0x592, 0x593, 0x3, 
    0x2, 0x2, 0x2, 0x593, 0x594, 0x3, 0x2, 0x2, 0x2, 0x594, 0x599, 0x5, 
    0x64, 0x33, 0x2, 0x595, 0x597, 0x7, 0x7f, 0x2, 0x2, 0x596, 0x595, 0x3, 
    0x2, 0x2, 0x2, 0x596, 0x597, 0x3, 0x2, 0x2, 0x2, 0x597, 0x598, 0x3, 
    0x2, 0x2, 0x2, 0x598, 0x59a, 0x5, 0xaa, 0x56, 0x2, 0x599, 0x596, 0x3, 
    0x2, 0x2, 0x2, 0x59a, 0x59b, 0x3, 0x2, 0x2, 0x2, 0x59b, 0x599, 0x3, 
    0x2, 0x2, 0x2, 0x59b, 0x59c, 0x3, 0x2, 0x2, 0x2, 0x59c, 0x59e, 0x3, 
    0x2, 0x2, 0x2, 0x59d, 0x587, 0x3, 0x2, 0x2, 0x2, 0x59d, 0x590, 0x3, 
    0x2, 0x2, 0x2, 0x59e, 0x5a7, 0x3, 0x2, 0x2, 0x2, 0x59f, 0x5a1, 0x7, 
    0x7f, 0x2, 0x2, 0x5a0, 0x59f, 0x3, 0x2, 0x2, 0x2, 0x5a0, 0x5a1, 0x3, 
    0x2, 0x2, 0x2, 0x5a1, 0x5a2, 0x3, 0x2, 0x2, 0x2, 0x5a2, 0x5a4, 0x7, 
    0x5d, 0x2, 0x2, 0x5a3, 0x5a5, 0x7, 0x7f, 0x2, 0x2, 0x5a4, 0x5a3, 0x3, 
    0x2, 0x2, 0x2, 0x5a4, 0x5a5, 0x3, 0x2, 0x2, 0x2, 0x5a5, 0x5a6, 0x3, 
    0x2, 0x2, 0x2, 0x5a6, 0x5a8, 0x5, 0x64, 0x33, 0x2, 0x5a7, 0x5a0, 0x3, 
    0x2, 0x2, 0x2, 0x5a7, 0x5a8, 0x3, 0x2, 0x2, 0x2, 0x5a8, 0x5aa, 0x3, 
    0x2, 0x2, 0x2, 0x5a9, 0x5ab, 0x7, 0x7f, 0x2, 0x2, 0x5aa, 0x5a9, 0x3, 
    0x2, 0x2, 0x2, 0x5aa, 0x5ab, 0x3, 0x2, 0x2, 0x2, 0x5ab, 0x5ac, 0x3, 
    0x2, 0x2, 0x2, 0x5ac, 0x5ad, 0x7, 0x5e, 0x2, 0x2, 0x5ad, 0xa9, 0x3, 
    0x2, 0x2, 0x2, 0x5ae, 0x5b0, 0x7, 0x5f, 0x2, 0x2, 0x5af, 0x5b1, 0x7, 
    0x7f, 0x2, 0x2, 0x5b0, 0x5af, 0x3, 0x2, 0x2, 0x2, 0x5b0, 0x5b1, 0x3, 
    0x2, 0x2, 0x2, 0x5b1, 0x5b2, 0x3, 0x2, 0x2, 0x2, 0x5b2, 0x5b4, 0x5, 
    0x64, 0x33, 0x2, 0x5b3, 0x5b5, 0x7, 0x7f, 0x2, 0x2, 0x5b4, 0x5b3, 0x3, 
    0x2, 0x2, 0x2, 0x5b4, 0x5b5, 0x3, 0x2, 0x2, 0x2, 0x5b5, 0x5b6, 0x3, 
    0x2, 0x2, 0x2, 0x5b6, 0x5b8, 0x7, 0x60, 0x2, 0x2, 0x5b7, 0x5b9, 0x7, 
    0x7f, 0x2, 0x2, 0x5b8, 0x5b7, 0x3, 0x2, 0x2, 0x2, 0x5b8, 0x5b9, 0x3, 
    0x2, 0x2, 0x2, 0x5b9, 0x5ba, 0x3, 0x2, 0x2, 0x2, 0x5ba, 0x5bb, 0x5, 
    0x64, 0x33, 0x2, 0x5bb, 0xab, 0x3, 0x2, 0x2, 0x2, 0x5bc, 0x5bd, 0x5, 
    0xc0, 0x61, 0x2, 0x5bd, 0xad, 0x3, 0x2, 0x2, 0x2, 0x5be, 0x5c1, 0x5, 
    0xba, 0x5e, 0x2, 0x5bf, 0x5c1, 0x5, 0xb8, 0x5d, 0x2, 0x5c0, 0x5be, 0x3, 
    0x2, 0x2, 0x2, 0x5c0, 0x5bf, 0x3, 0x2, 0x2, 0x2, 0x5c1, 0xaf, 0x3, 0x2, 
    0x2, 0x2, 0x5c2, 0x5c4, 0x7, 0x1a, 0x2, 0x2, 0x5c3, 0x5c5, 0x7, 0x7f, 
    0x2, 0x2, 0x5c4, 0x5c3, 0x3, 0x2, 0x2, 0x2, 0x5c4, 0x5c5, 0x3, 0x2, 
    0x2, 0x2, 0x5c5, 0x5e7, 0x3, 0x2, 0x2, 0x2, 0x5c6, 0x5c8, 0x5, 0xb6, 
    0x5c, 0x2, 0x5c7, 0x5c9, 0x7, 0x7f, 0x2, 0x2, 0x5c8, 0x5c7, 0x3, 0x2, 
    0x2, 0x2, 0x5c8, 0x5c9, 0x3, 0x2, 0x2, 0x2, 0x5c9, 0x5ca, 0x3, 0x2, 
    0x2, 0x2, 0x5ca, 0x5cc, 0x7, 0xc, 0x2, 0x2, 0x5cb, 0x5cd, 0x7, 0x7f, 
    0x2, 0x2, 0x5cc, 0x5cb, 0x3, 0x2, 0x2, 0x2, 0x5cc, 0x5cd, 0x3, 0x2, 
    0x2, 0x2, 0x5cd, 0x5ce, 0x3, 0x2, 0x2, 0x2, 0x5ce, 0x5d0, 0x5, 0x64, 
    0x33, 0x2, 0x5cf, 0x5d1, 0x7, 0x7f, 0x2, 0x2, 0x5d0, 0x5cf, 0x3, 0x2, 
    0x2, 0x2, 0x5d0, 0x5d1, 0x3, 0x2, 0x2, 0x2, 0x5d1, 0x5e4, 0x3, 0x2, 
    0x2, 0x2, 0x5d2, 0x5d4, 0x7, 0x4, 0x2, 0x2, 0x5d3, 0x5d5, 0x7, 0x7f, 
    0x2, 0x2, 0x5d4, 0x5d3, 0x3, 0x2, 0x2, 0x2, 0x5d4, 0x5d5, 0x3, 0x2, 
    0x2, 0x2, 0x5d5, 0x5d6, 0x3, 0x2, 0x2, 0x2, 0x5d6, 0x5d8, 0x5, 0xb6, 
    0x5c, 0x2, 0x5d7, 0x5d9, 0x7, 0x7f, 0x2, 0x2, 0x5d8, 0x5d7, 0x3, 0x2, 
    0x2, 0x2, 0x5d8, 0x5d9, 0x3, 0x2, 0x2, 0x2, 0x5d9, 0x5da, 0x3, 0x2, 
    0x2, 0x2, 0x5da, 0x5dc, 0x7, 0xc, 0x2, 0x2, 0x5db, 0x5dd, 0x7, 0x7f, 
    0x2, 0x2, 0x5dc, 0x5db, 0x3, 0x2, 0x2, 0x2, 0x5dc, 0x5dd, 0x3, 0x2, 
    0x2, 0x2, 0x5dd, 0x5de, 0x3, 0x2, 0x2, 0x2, 0x5de, 0x5e0, 0x5, 0x64, 
    0x33, 0x2, 0x5df, 0x5e1, 0x7, 0x7f, 0x2, 0x2, 0x5e0, 0x5df, 0x3, 0x2, 
    0x2, 0x2, 0x5e0, 0x5e1, 0x3, 0x2, 0x2, 0x2, 0x5e1, 0x5e3, 0x3, 0x2, 
    0x2, 0x2, 0x5e2, 0x5d2, 0x3, 0x2, 0x2, 0x2, 0x5e3, 0x5e6, 0x3, 0x2, 
    0x2, 0x2, 0x5e4, 0x5e2, 0x3, 0x2, 0x2, 0x2, 0x5e4, 0x5e5, 0x3, 0x2, 
    0x2, 0x2, 0x5e5, 0x5e8, 0x3, 0x2, 0x2, 0x2, 0x5e6, 0x5e4, 0x3, 0x2, 
    0x2, 0x2, 0x5e7, 0x5c6, 0x3, 0x2, 0x2, 0x2, 0x5e7, 0x5e8, 0x3, 0x2, 
    0x2, 0x2, 0x5e8, 0x5e9, 0x3, 0x2, 0x2, 0x2, 0x5e9, 0x5ea, 0x7, 0x1b, 
    0x2, 0x2, 0x5ea, 0xb1, 0x3, 0x2, 0x2, 0x2, 0x5eb, 0x5ee, 0x7, 0x1c, 
    0x2, 0x2, 0x5ec, 0x5ef, 0x5, 0xc0, 0x61, 0x2, 0x5ed, 0x5ef, 0x7, 0x64, 
    0x2, 0x2, 0x5ee, 0x5ec, 0x3, 0x2, 0x2, 0x2, 0x5ee, 0x5ed, 0x3, 0x2, 
    0x2, 0x2, 0x5ef, 0xb3, 0x3, 0x2, 0x2, 0x2, 0x5f0, 0x5f5, 0x5, 0x82, 
    0x42, 0x2, 0x5f1, 0x5f3, 0x7, 0x7f, 0x2, 0x2, 0x5f2, 0x5f1, 0x3, 0x2, 
    0x2, 0x2, 0x5f2, 0x5f3, 0x3, 0x2, 0x2, 0x2, 0x5f3, 0x5f4, 0x3, 0x2, 
    0x2, 0x2, 0x5f4, 0x5f6, 0x5, 0xa6, 0x54, 0x2, 0x5f5, 0x5f2, 0x3, 0x2, 
    0x2, 0x2, 0x5f6, 0x5f7, 0x3, 0x2, 0x2, 0x2, 0x5f7, 0x5f5, 0x3, 0x2, 
    0x2, 0x2, 0x5f7, 0x5f8, 0x3, 0x2, 0x2, 0x2, 0x5f8, 0xb5, 0x3, 0x2, 0x2, 
    0x2, 0x5f9, 0x5fa, 0x5, 0xbc, 0x5f, 0x2, 0x5fa, 0xb7, 0x3, 0x2, 0x2, 
    0x2, 0x5fb, 0x5fc, 0x9, 0x5, 0x2, 0x2, 0x5fc, 0xb9, 0x3, 0x2, 0x2, 0x2, 
    0x5fd, 0x5fe, 0x9, 0x6, 0x2, 0x2, 0x5fe, 0xbb, 0x3, 0x2, 0x2, 0x2, 0x5ff, 
    0x602, 0x5, 0xc0, 0x61, 0x2, 0x600, 0x602, 0x5, 0xbe, 0x60, 0x2, 0x601, 
    0x5ff, 0x3, 0x2, 0x2, 0x2, 0x601, 0x600, 0x3, 0x2, 0x2, 0x2, 0x602, 
    0xbd, 0x3, 0x2, 0x2, 0x2, 0x603, 0x604, 0x9, 0x7, 0x2, 0x2, 0x604, 0xbf, 
    0x3, 0x2, 0x2, 0x2, 0x605, 0x606, 0x9, 0x8, 0x2, 0x2, 0x606, 0xc1, 0x3, 
    0x2, 0x2, 0x2, 0x607, 0x608, 0x9, 0x9, 0x2, 0x2, 0x608, 0xc3, 0x3, 0x2, 
    0x2, 0x2, 0x609, 0x60a, 0x9, 0xa, 0x2, 0x2, 0x60a, 0xc5, 0x3, 0x2, 0x2, 
    0x2, 0x60b, 0x60c, 0x9, 0xb, 0x2, 0x2, 0x60c, 0xc7, 0x3, 0x2, 0x2, 0x2, 
    0x11f, 0xc9, 0xcd, 0xd0, 0xd3, 0xdb, 0xdf, 0xe4, 0xeb, 0xf0, 0xf3, 0xf7, 
    0xfb, 0xff, 0x105, 0x109, 0x10e, 0x113, 0x117, 0x11a, 0x11c, 0x120, 
    0x124, 0x129, 0x12d, 0x132, 0x136, 0x13f, 0x144, 0x148, 0x14c, 0x150, 
    0x153, 0x157, 0x161, 0x168, 0x175, 0x179, 0x17f, 0x186, 0x18b, 0x18f, 
    0x195, 0x199, 0x19f, 0x1a3, 0x1a9, 0x1ad, 0x1b1, 0x1b5, 0x1b9, 0x1bd, 
    0x1c2, 0x1c9, 0x1cd, 0x1d2, 0x1d9, 0x1df, 0x1e4, 0x1ea, 0x1f0, 0x1f5, 
    0x1f9, 0x1fe, 0x201, 0x204, 0x207, 0x20e, 0x214, 0x217, 0x21c, 0x21f, 
    0x223, 0x226, 0x22e, 0x232, 0x236, 0x23a, 0x23e, 0x243, 0x248, 0x24c, 
    0x251, 0x254, 0x25d, 0x266, 0x26b, 0x278, 0x27b, 0x283, 0x287, 0x28c, 
    0x291, 0x295, 0x29a, 0x2a0, 0x2a5, 0x2ac, 0x2b0, 0x2b4, 0x2b6, 0x2ba, 
    0x2bc, 0x2c0, 0x2c2, 0x2c8, 0x2ce, 0x2d2, 0x2d5, 0x2d8, 0x2dc, 0x2e2, 
    0x2e6, 0x2e9, 0x2ec, 0x2f2, 0x2f5, 0x2f8, 0x2fc, 0x302, 0x305, 0x308, 
    0x30c, 0x310, 0x314, 0x316, 0x31a, 0x31c, 0x31f, 0x323, 0x325, 0x32b, 
    0x32f, 0x333, 0x337, 0x33a, 0x33f, 0x344, 0x349, 0x34e, 0x354, 0x358, 
    0x35a, 0x35e, 0x362, 0x364, 0x366, 0x375, 0x37f, 0x389, 0x38e, 0x392, 
    0x399, 0x39e, 0x3a3, 0x3a7, 0x3ab, 0x3af, 0x3b2, 0x3b4, 0x3b9, 0x3bd, 
    0x3c1, 0x3c5, 0x3c9, 0x3cd, 0x3d0, 0x3d2, 0x3d7, 0x3db, 0x3e0, 0x3e5, 
    0x3e9, 0x3f2, 0x3f4, 0x3fa, 0x3fe, 0x405, 0x409, 0x40d, 0x410, 0x41c, 
    0x41f, 0x42d, 0x431, 0x436, 0x43a, 0x43d, 0x444, 0x448, 0x44c, 0x453, 
    0x457, 0x45b, 0x461, 0x465, 0x469, 0x46f, 0x473, 0x477, 0x47d, 0x481, 
    0x485, 0x48d, 0x495, 0x49b, 0x49f, 0x4a3, 0x4a7, 0x4ab, 0x4ae, 0x4b4, 
    0x4b9, 0x4be, 0x4c3, 0x4c8, 0x4cd, 0x4d0, 0x4d4, 0x4d8, 0x4de, 0x4e3, 
    0x4e7, 0x4ea, 0x4f4, 0x4f8, 0x4fc, 0x4fe, 0x502, 0x506, 0x50a, 0x50e, 
    0x511, 0x519, 0x51d, 0x521, 0x525, 0x529, 0x52d, 0x531, 0x534, 0x544, 
    0x549, 0x54d, 0x551, 0x554, 0x557, 0x55d, 0x561, 0x565, 0x567, 0x56b, 
    0x56f, 0x573, 0x575, 0x579, 0x57d, 0x583, 0x589, 0x58e, 0x592, 0x596, 
    0x59b, 0x59d, 0x5a0, 0x5a4, 0x5a7, 0x5aa, 0x5b0, 0x5b4, 0x5b8, 0x5c0, 
    0x5c4, 0x5c8, 0x5cc, 0x5d0, 0x5d4, 0x5d8, 0x5dc, 0x5e0, 0x5e4, 0x5e7, 
    0x5ee, 0x5f2, 0x5f7, 0x601, 
  };

  atn::ATNDeserializer deserializer;
  _atn = deserializer.deserialize(_serializedATN);

  size_t count = _atn.getNumberOfDecisions();
  _decisionToDFA.reserve(count);
  for (size_t i = 0; i < count; i++) { 
    _decisionToDFA.emplace_back(_atn.getDecisionState(i), i);
  }
}

CypherParser::Initializer CypherParser::_init;
