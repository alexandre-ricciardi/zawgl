// CypherEngineImpl.cpp : D�finit les fonctions export�es pour l'application DLL.
//

#include "CypherEngineImpl.h"
#include <iostream>
CypherEngineImpl::CypherEngineImpl()
{

}

void CypherEngineImpl::iterateVariable(CypherParser::OC_VariableContext* var)
{
	std::cout << var->oC_SymbolicName()->getText();
}

void CypherEngineImpl::iteratePropertyKeyName(CypherParser::OC_PropertyKeyNameContext* pkey)
{
	std::cout << pkey->oC_SchemaName()->getText();
}

void CypherEngineImpl::iterateLiteral(CypherParser::OC_LiteralContext* lit)
{
	auto str = lit->StringLiteral();
	if (str) {
		std::cout << str->getText();
	}
}

void CypherEngineImpl::iterateAtom(CypherParser::OC_AtomContext* atom)
{
	auto lit = atom->oC_Literal();
	if (lit) {
		iterateLiteral(lit);
	}
}

void CypherEngineImpl::iteratePropertyOrLabelsExpression(CypherParser::OC_PropertyOrLabelsExpressionContext* pol)
{
	auto atom = pol->oC_Atom();
	if (atom) {
		iterateAtom(atom);
	}
}

void CypherEngineImpl::iterateStringOperatorExpression(CypherParser::OC_StringOperatorExpressionContext* str)
{
	auto azre = str->oC_PropertyOrLabelsExpression();
}

void CypherEngineImpl::iterateStringListNullOperatorExpression(CypherParser::OC_StringListNullOperatorExpressionContext* sno)
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

void CypherEngineImpl::iterateUnaryAddOrSubtractExpression(CypherParser::OC_UnaryAddOrSubtractExpressionContext* uase)
{
	auto sno = uase->oC_StringListNullOperatorExpression();
	if (sno) {
		iterateStringListNullOperatorExpression(sno);
	}
}

void CypherEngineImpl::iteratePowerOfExpression(CypherParser::OC_PowerOfExpressionContext* poe)
{
	auto uases = poe->oC_UnaryAddOrSubtractExpression();
	for (auto uase : uases) {
		iterateUnaryAddOrSubtractExpression(uase);
	}
}

void CypherEngineImpl::iterateMultiplyDivideModuloExpression(CypherParser::OC_MultiplyDivideModuloExpressionContext* mdme)
{
	auto poes = mdme->oC_PowerOfExpression();
	for (auto poe : poes) {
		iteratePowerOfExpression(poe);
	}
}

void CypherEngineImpl::iterateComparisonExpression(CypherParser::OC_ComparisonExpressionContext* cxp)
{
	auto ase = cxp->oC_AddOrSubtractExpression();
	if (ase) {
		auto mdmes = ase->oC_MultiplyDivideModuloExpression();
		for (auto mdme : mdmes) {
			iterateMultiplyDivideModuloExpression(mdme);
		}
	}
}

void CypherEngineImpl::iterateXorExpression(CypherParser::OC_XorExpressionContext* xor_)
{
	auto axps = xor_ ->oC_AndExpression();
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

void CypherEngineImpl::iterateExpression(CypherParser::OC_ExpressionContext* expr)
{
	auto oxp = expr->oC_OrExpression();
	if (oxp) {
		auto xorxps = oxp->oC_XorExpression();
		for (auto xor_ : xorxps) {
			iterateXorExpression(xor_);
		}
	}
}

void CypherEngineImpl::iterateMapLiteral(CypherParser::OC_MapLiteralContext* map)
{
	auto pkeys = map->oC_PropertyKeyName();
	auto exprs = map->oC_Expression();
	for (int i = 0; i < pkeys.size(); ++i) {
		currentProperty = {};
		auto keyName = pkeys[i];
		auto expr = exprs[i];
		iteratePropertyKeyName(keyName);
		iterateExpression(expr);
		currentProperties.push_back(currentProperty);
	}
}

void CypherEngineImpl::iterateProperties(CypherParser::OC_PropertiesContext* props)
{
	currentProperties = {};
	auto map = props->oC_MapLiteral();
	if (map) {
		iterateMapLiteral(map);
	}
}

void CypherEngineImpl::iterateNodePattern(CypherParser::OC_NodePatternContext* npat)
{
	currentNodePattern = {};
	auto props = npat->oC_Properties();
	if (props) {
		iterateProperties(props);
	}
	currentNodePattern.properties = currentProperties;
	auto var = npat->oC_Variable();
	if (var) {
		iterateVariable(var);
	}
	
}

void CypherEngineImpl::iteratePatternElement(CypherParser::OC_PatternElementContext* elt)
{
	auto np = elt->oC_NodePattern();
	if (np) {
		iterateNodePattern(np);
	}
}

void CypherEngineImpl::iterateAnonPattern(CypherParser::OC_AnonymousPatternPartContext* pat)
{
	auto elt = pat->oC_PatternElement();
	if (elt) {
		iteratePatternElement(elt);
	}
}

void CypherEngineImpl::iteratePattern(CypherParser::OC_PatternContext* pattern)
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

void CypherEngineImpl::iterateCreate(CypherParser::OC_CreateContext* create)
{
	auto pattern = create->oC_Pattern();
	if (pattern) {
		currentGraphPattern = {};
		iteratePattern(pattern);
		model.createGraphs.push_back(currentGraphPattern);
	}
}

void CypherEngineImpl::iterateSinglePartQuery(CypherParser::OC_SinglePartQueryContext* sp)
{
	auto updates = sp->oC_UpdatingClause();
	for (auto update : updates) {
		auto create = update->oC_Create();
		if (create) {
			iterateCreate(create);
		}
	}
}

void CypherEngineImpl::iterateSingleQuery(CypherParser::OC_SingleQueryContext* sq)
{
	auto singlePartQuery = sq->oC_SinglePartQuery();
	if (singlePartQuery) {
		iterateSinglePartQuery(singlePartQuery);
	}
}

void CypherEngineImpl::iterateRegularQuery(CypherParser::OC_RegularQueryContext* rq)
{
	auto singleQuery = rq->oC_SingleQuery();
	if (singleQuery) {
		iterateSingleQuery(singleQuery);
	}
}

void CypherEngineImpl::iterateCypherTree(CypherParser::OC_CypherContext* tree)
{
	auto stmt = tree->oC_Statement();
	auto query = stmt->oC_Query();
	auto regularQuery = query->oC_RegularQuery();
	if (regularQuery) {
		iterateRegularQuery(regularQuery);
	}
}

void CypherEngineImpl::process(const std::string& expr)
{
	std::stringstream stream(expr);
	ANTLRInputStream input(stream);
	CypherLexer lexer(&input);
	CommonTokenStream tokens(&lexer);
	CypherParser parser(&tokens);
	auto tree = parser.oC_Cypher();
	iterateCypherTree(tree);
}

