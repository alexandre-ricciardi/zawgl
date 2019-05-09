// CypherEngine.cpp : Définit les fonctions exportées pour l'application DLL.
//

#include "CypherEngine.h"
#include <iostream>
#include "antlr4-runtime.h"
#include "CypherLexer.h"
#include "CypherParser.h"
using namespace antlr4;
CypherEngine::CypherEngine(GraphRepository& gr)
{

}

void iterateVariable(CypherParser::OC_VariableContext* var)
{
	std::cout << var->oC_SymbolicName()->getText();
}

void iteratePropertyKeyName(CypherParser::OC_PropertyKeyNameContext* pkey)
{
	std::cout << pkey->oC_SchemaName()->getText();
}

void iterateLiteral(CypherParser::OC_LiteralContext* lit)
{
	auto str = lit->StringLiteral();
	if (str) {
		std::cout << str->getText();
	}
}

void iterateAtom(CypherParser::OC_AtomContext* atom)
{
	auto lit = atom->oC_Literal();
	if (lit) {
		iterateLiteral(lit);
	}
}

void iteratePropertyOrLabelsExpression(CypherParser::OC_PropertyOrLabelsExpressionContext* pol)
{
	auto atom = pol->oC_Atom();
	if (atom) {
		iterateAtom(atom);
	}
}

void iterateStringOperatorExpression(CypherParser::OC_StringOperatorExpressionContext* str)
{
	auto azre = str->oC_PropertyOrLabelsExpression();
}

void iterateStringListNullOperatorExpression(CypherParser::OC_StringListNullOperatorExpressionContext* sno)
{
	auto pol = sno->oC_PropertyOrLabelsExpression();
	if (pol) {
		iteratePropertyOrLabelsExpression(pol);
	}
	auto strs = sno->oC_StringOperatorExpression();
	for (auto str : strs) {
		iterateStringOperatorExpression(str);
	}
}

void iterateUnaryAddOrSubtractExpression(CypherParser::OC_UnaryAddOrSubtractExpressionContext* uase)
{
	auto sno = uase->oC_StringListNullOperatorExpression();
	if (sno) {
		iterateStringListNullOperatorExpression(sno);
	}
}

void iteratePowerOfExpression(CypherParser::OC_PowerOfExpressionContext* poe)
{
	auto uases = poe->oC_UnaryAddOrSubtractExpression();
	for (auto uase : uases) {
		iterateUnaryAddOrSubtractExpression(uase);
	}
}

void iterateMultiplyDivideModuloExpression(CypherParser::OC_MultiplyDivideModuloExpressionContext* mdme)
{
	auto poes = mdme->oC_PowerOfExpression();
	for (auto poe : poes) {
		iteratePowerOfExpression(poe);
	}
}

void iterateComparisonExpression(CypherParser::OC_ComparisonExpressionContext* cxp)
{
	auto ase = cxp->oC_AddOrSubtractExpression();
	if (ase) {
		auto mdmes = ase->oC_MultiplyDivideModuloExpression();
		for (auto mdme : mdmes) {
			iterateMultiplyDivideModuloExpression(mdme);
		}
	}
}

void iterateXorExpression(CypherParser::OC_XorExpressionContext*xor)
{
	auto axps = xor ->oC_AndExpression();
	for (auto axp : axps) {
		auto nxps = axp->oC_NotExpression();
		for (auto nxp : nxps) {
			auto cxp = nxp->oC_ComparisonExpression();
			if (cxp) {
				iterateComparisonExpression(cxp);
			}
		}
	}
}

void iterateExpression(CypherParser::OC_ExpressionContext* expr)
{
	auto oxp = expr->oC_OrExpression();
	if (oxp) {
		auto xorxps = oxp->oC_XorExpression();
		for (auto xor : xorxps) {
			iterateXorExpression(xor);
		}
	}
}

void iterateMapLiteral(CypherParser::OC_MapLiteralContext* map)
{
	auto pkeys = map->oC_PropertyKeyName();
	for (auto keyName : pkeys) {
		iteratePropertyKeyName(keyName);
	}
	auto exprs = map->oC_Expression();
	for (auto expr : exprs) {
		iterateExpression(expr);
	}
}

void iterateProperties(CypherParser::OC_PropertiesContext* props)
{
	auto map = props->oC_MapLiteral();
	if (map) {
		iterateMapLiteral(map);
	}
}

void iterateNodePattern(CypherParser::OC_NodePatternContext* npat)
{
	auto props = npat->oC_Properties();
	if (props) {
		iterateProperties(props);
	}
	auto var = npat->oC_Variable();
	if (var) {
		iterateVariable(var);
	}
}

void iteratePatternElement(CypherParser::OC_PatternElementContext* elt)
{
	auto np = elt->oC_NodePattern();
	if (np) {
		iterateNodePattern(np);
	}
}

void iterateAnonPattern(CypherParser::OC_AnonymousPatternPartContext* pat)
{
	auto elt = pat->oC_PatternElement();
	if (elt) {
		iteratePatternElement(elt);
	}
}

void iteratePattern(CypherParser::OC_PatternContext* pattern)
{
	for (auto part : pattern->oC_PatternPart()) {
		auto var = part->oC_Variable();
		if (var) {
			iterateVariable(var);
		}
		auto pat = part->oC_AnonymousPatternPart();
		if (pat) {
			iterateAnonPattern(pat);
		}
	}
}

void iterateCreate(CypherParser::OC_CreateContext* create)
{
	auto pattern = create->oC_Pattern();
	if (pattern) {
		iteratePattern(pattern);
	}
}

void iterateSinglePartQuery(CypherParser::OC_SinglePartQueryContext* sp)
{
	auto updates = sp->oC_UpdatingClause();
	for (auto update : updates) {
		auto create = update->oC_Create();
		if (create) {
			iterateCreate(create);
		}
	}
}

void iterateSingleQuery(CypherParser::OC_SingleQueryContext* sq)
{
	auto singlePartQuery = sq->oC_SinglePartQuery();
	if (singlePartQuery) {
		iterateSinglePartQuery(singlePartQuery);
	}
}

void iterateRegularQuery(CypherParser::OC_RegularQueryContext* rq)
{
	auto singleQuery = rq->oC_SingleQuery();
	if (singleQuery) {
		iterateSingleQuery(singleQuery);
	}
}

void iterateCypherTree(CypherParser::OC_CypherContext* tree)
{
	auto stmt = tree->oC_Statement();
	auto query = stmt->oC_Query();
	auto regularQuery = query->oC_RegularQuery();
	if (regularQuery) {
		iterateRegularQuery(regularQuery);
	}
}

void CypherEngine::process(const std::string & expr)
{
	std::stringstream stream(expr);
	ANTLRInputStream input(stream);
	CypherLexer lexer(&input);
	CommonTokenStream tokens(&lexer);
	CypherParser parser(&tokens);
	auto tree = parser.oC_Cypher();
	iterateCypherTree(tree);
}

