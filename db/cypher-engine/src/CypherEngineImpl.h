#pragma once

#include <string>

#include "GraphRepository.h"
#include "antlr4-runtime.h"
#include "CypherLexer.h"
#include "CypherParser.h"
#include "CypherModel.h"
using namespace antlr4;

class CypherEngineImpl {
private:
	CypherModel model;
	void iterateCypherTree(CypherParser::OC_CypherContext* tree);
	void iterateRegularQuery(CypherParser::OC_RegularQueryContext* rq);
	void iterateSingleQuery(CypherParser::OC_SingleQueryContext* sq);
	void iterateSinglePartQuery(CypherParser::OC_SinglePartQueryContext* sp);
	void iterateCreate(CypherParser::OC_CreateContext* create);
	void iteratePattern(CypherParser::OC_PatternContext* pattern);
	void iterateAnonPattern(CypherParser::OC_AnonymousPatternPartContext* pat);
	void iteratePatternElement(CypherParser::OC_PatternElementContext* elt);
	void iterateNodePattern(CypherParser::OC_NodePatternContext* npat);
	void iterateProperties(CypherParser::OC_PropertiesContext* props);
	void iterateMapLiteral(CypherParser::OC_MapLiteralContext* map);
	void iterateExpression(CypherParser::OC_ExpressionContext* expr);
	void iterateXorExpression(CypherParser::OC_XorExpressionContext*xor);
	void iterateComparisonExpression(CypherParser::OC_ComparisonExpressionContext* cxp);
	void iterateMultiplyDivideModuloExpression(CypherParser::OC_MultiplyDivideModuloExpressionContext* mdme);
	void iteratePowerOfExpression(CypherParser::OC_PowerOfExpressionContext* poe);
	void iterateUnaryAddOrSubtractExpression(CypherParser::OC_UnaryAddOrSubtractExpressionContext* uase);
	void iterateStringListNullOperatorExpression(CypherParser::OC_StringListNullOperatorExpressionContext* sno);
	void iterateStringOperatorExpression(CypherParser::OC_StringOperatorExpressionContext* str);
	void iteratePropertyOrLabelsExpression(CypherParser::OC_PropertyOrLabelsExpressionContext* pol);
	void iterateAtom(CypherParser::OC_AtomContext* atom);
	void iterateLiteral(CypherParser::OC_LiteralContext* lit);
	void iteratePropertyKeyName(CypherParser::OC_PropertyKeyNameContext* pkey);
	void iterateVariable(CypherParser::OC_VariableContext* var);
public:
	CypherEngineImpl(GraphRepository& gr);
	void process(const std::string& expr);
};