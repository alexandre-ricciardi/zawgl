
// Generated from Cypher.g4 by ANTLR 4.7.2

#pragma once


#include "antlr4-runtime.h"




class  __declspec(dllexport) CypherLexer : public antlr4::Lexer {
public:
  enum {
    T__0 = 1, T__1 = 2, T__2 = 3, T__3 = 4, T__4 = 5, T__5 = 6, T__6 = 7, 
    T__7 = 8, T__8 = 9, T__9 = 10, T__10 = 11, T__11 = 12, T__12 = 13, T__13 = 14, 
    T__14 = 15, T__15 = 16, T__16 = 17, T__17 = 18, T__18 = 19, T__19 = 20, 
    T__20 = 21, T__21 = 22, T__22 = 23, T__23 = 24, T__24 = 25, T__25 = 26, 
    T__26 = 27, T__27 = 28, T__28 = 29, T__29 = 30, T__30 = 31, T__31 = 32, 
    T__32 = 33, T__33 = 34, T__34 = 35, T__35 = 36, T__36 = 37, T__37 = 38, 
    T__38 = 39, T__39 = 40, T__40 = 41, T__41 = 42, T__42 = 43, T__43 = 44, 
    T__44 = 45, UNION = 46, ALL = 47, OPTIONAL = 48, MATCH = 49, UNWIND = 50, 
    AS = 51, MERGE = 52, ON = 53, CREATE = 54, SET = 55, DETACH = 56, DELETE = 57, 
    REMOVE = 58, CALL = 59, YIELD = 60, WITH = 61, DISTINCT = 62, RETURN = 63, 
    ORDER = 64, BY = 65, L_SKIP = 66, LIMIT = 67, ASCENDING = 68, ASC = 69, 
    DESCENDING = 70, DESC = 71, WHERE = 72, OR = 73, XOR = 74, AND = 75, 
    NOT = 76, IN = 77, STARTS = 78, ENDS = 79, CONTAINS = 80, IS = 81, NULLOP = 82, 
    COUNT = 83, ANY = 84, NONE = 85, SINGLE = 86, TRUE = 87, FALSE = 88, 
    EXISTS = 89, CASE = 90, ELSE = 91, END = 92, WHEN = 93, THEN = 94, StringLiteral = 95, 
    EscapedChar = 96, HexInteger = 97, DecimalInteger = 98, OctalInteger = 99, 
    HexLetter = 100, HexDigit = 101, Digit = 102, NonZeroDigit = 103, NonZeroOctDigit = 104, 
    OctDigit = 105, ZeroDigit = 106, ExponentDecimalReal = 107, RegularDecimalReal = 108, 
    CONSTRAINT = 109, DO = 110, FOR = 111, REQUIRE = 112, UNIQUE = 113, 
    MANDATORY = 114, SCALAR = 115, OF = 116, ADD = 117, DROP = 118, FILTER = 119, 
    EXTRACT = 120, UnescapedSymbolicName = 121, IdentifierStart = 122, IdentifierPart = 123, 
    EscapedSymbolicName = 124, SP = 125, WHITESPACE = 126, Comment = 127
  };

  CypherLexer(antlr4::CharStream *input);
  ~CypherLexer();

  virtual std::string getGrammarFileName() const override;
  virtual const std::vector<std::string>& getRuleNames() const override;

  virtual const std::vector<std::string>& getChannelNames() const override;
  virtual const std::vector<std::string>& getModeNames() const override;
  virtual const std::vector<std::string>& getTokenNames() const override; // deprecated, use vocabulary instead
  virtual antlr4::dfa::Vocabulary& getVocabulary() const override;

  virtual const std::vector<uint16_t> getSerializedATN() const override;
  virtual const antlr4::atn::ATN& getATN() const override;

private:
  static std::vector<antlr4::dfa::DFA> _decisionToDFA;
  static antlr4::atn::PredictionContextCache _sharedContextCache;
  static std::vector<std::string> _ruleNames;
  static std::vector<std::string> _tokenNames;
  static std::vector<std::string> _channelNames;
  static std::vector<std::string> _modeNames;

  static std::vector<std::string> _literalNames;
  static std::vector<std::string> _symbolicNames;
  static antlr4::dfa::Vocabulary _vocabulary;
  static antlr4::atn::ATN _atn;
  static std::vector<uint16_t> _serializedATN;


  // Individual action functions triggered by action() above.

  // Individual semantic predicate functions triggered by sempred() above.

  struct Initializer {
    Initializer();
  };
  static Initializer _init;
};

