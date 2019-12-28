
// Generated from Cypher.g4 by ANTLR 4.7.2

#pragma once


#include "antlr4-runtime.h"
#include "CypherListener.h"


/**
 * This class provides an empty implementation of CypherListener,
 * which can be extended to create a listener which only needs to handle a subset
 * of the available methods.
 */
class  CypherBaseListener : public CypherListener {
public:

  virtual void enterOC_Cypher(CypherParser::OC_CypherContext * /*ctx*/) override { }
  virtual void exitOC_Cypher(CypherParser::OC_CypherContext * /*ctx*/) override { }

  virtual void enterOC_Statement(CypherParser::OC_StatementContext * /*ctx*/) override { }
  virtual void exitOC_Statement(CypherParser::OC_StatementContext * /*ctx*/) override { }

  virtual void enterOC_Query(CypherParser::OC_QueryContext * /*ctx*/) override { }
  virtual void exitOC_Query(CypherParser::OC_QueryContext * /*ctx*/) override { }

  virtual void enterOC_RegularQuery(CypherParser::OC_RegularQueryContext * /*ctx*/) override { }
  virtual void exitOC_RegularQuery(CypherParser::OC_RegularQueryContext * /*ctx*/) override { }

  virtual void enterOC_Union(CypherParser::OC_UnionContext * /*ctx*/) override { }
  virtual void exitOC_Union(CypherParser::OC_UnionContext * /*ctx*/) override { }

  virtual void enterOC_SingleQuery(CypherParser::OC_SingleQueryContext * /*ctx*/) override { }
  virtual void exitOC_SingleQuery(CypherParser::OC_SingleQueryContext * /*ctx*/) override { }

  virtual void enterOC_SinglePartQuery(CypherParser::OC_SinglePartQueryContext * /*ctx*/) override { }
  virtual void exitOC_SinglePartQuery(CypherParser::OC_SinglePartQueryContext * /*ctx*/) override { }

  virtual void enterOC_MultiPartQuery(CypherParser::OC_MultiPartQueryContext * /*ctx*/) override { }
  virtual void exitOC_MultiPartQuery(CypherParser::OC_MultiPartQueryContext * /*ctx*/) override { }

  virtual void enterOC_UpdatingClause(CypherParser::OC_UpdatingClauseContext * /*ctx*/) override { }
  virtual void exitOC_UpdatingClause(CypherParser::OC_UpdatingClauseContext * /*ctx*/) override { }

  virtual void enterOC_ReadingClause(CypherParser::OC_ReadingClauseContext * /*ctx*/) override { }
  virtual void exitOC_ReadingClause(CypherParser::OC_ReadingClauseContext * /*ctx*/) override { }

  virtual void enterOC_Match(CypherParser::OC_MatchContext * /*ctx*/) override { }
  virtual void exitOC_Match(CypherParser::OC_MatchContext * /*ctx*/) override { }

  virtual void enterOC_Unwind(CypherParser::OC_UnwindContext * /*ctx*/) override { }
  virtual void exitOC_Unwind(CypherParser::OC_UnwindContext * /*ctx*/) override { }

  virtual void enterOC_Merge(CypherParser::OC_MergeContext * /*ctx*/) override { }
  virtual void exitOC_Merge(CypherParser::OC_MergeContext * /*ctx*/) override { }

  virtual void enterOC_MergeAction(CypherParser::OC_MergeActionContext * /*ctx*/) override { }
  virtual void exitOC_MergeAction(CypherParser::OC_MergeActionContext * /*ctx*/) override { }

  virtual void enterOC_Create(CypherParser::OC_CreateContext * /*ctx*/) override { }
  virtual void exitOC_Create(CypherParser::OC_CreateContext * /*ctx*/) override { }

  virtual void enterOC_Set(CypherParser::OC_SetContext * /*ctx*/) override { }
  virtual void exitOC_Set(CypherParser::OC_SetContext * /*ctx*/) override { }

  virtual void enterOC_SetItem(CypherParser::OC_SetItemContext * /*ctx*/) override { }
  virtual void exitOC_SetItem(CypherParser::OC_SetItemContext * /*ctx*/) override { }

  virtual void enterOC_Delete(CypherParser::OC_DeleteContext * /*ctx*/) override { }
  virtual void exitOC_Delete(CypherParser::OC_DeleteContext * /*ctx*/) override { }

  virtual void enterOC_Remove(CypherParser::OC_RemoveContext * /*ctx*/) override { }
  virtual void exitOC_Remove(CypherParser::OC_RemoveContext * /*ctx*/) override { }

  virtual void enterOC_RemoveItem(CypherParser::OC_RemoveItemContext * /*ctx*/) override { }
  virtual void exitOC_RemoveItem(CypherParser::OC_RemoveItemContext * /*ctx*/) override { }

  virtual void enterOC_InQueryCall(CypherParser::OC_InQueryCallContext * /*ctx*/) override { }
  virtual void exitOC_InQueryCall(CypherParser::OC_InQueryCallContext * /*ctx*/) override { }

  virtual void enterOC_StandaloneCall(CypherParser::OC_StandaloneCallContext * /*ctx*/) override { }
  virtual void exitOC_StandaloneCall(CypherParser::OC_StandaloneCallContext * /*ctx*/) override { }

  virtual void enterOC_YieldItems(CypherParser::OC_YieldItemsContext * /*ctx*/) override { }
  virtual void exitOC_YieldItems(CypherParser::OC_YieldItemsContext * /*ctx*/) override { }

  virtual void enterOC_YieldItem(CypherParser::OC_YieldItemContext * /*ctx*/) override { }
  virtual void exitOC_YieldItem(CypherParser::OC_YieldItemContext * /*ctx*/) override { }

  virtual void enterOC_With(CypherParser::OC_WithContext * /*ctx*/) override { }
  virtual void exitOC_With(CypherParser::OC_WithContext * /*ctx*/) override { }

  virtual void enterOC_Return(CypherParser::OC_ReturnContext * /*ctx*/) override { }
  virtual void exitOC_Return(CypherParser::OC_ReturnContext * /*ctx*/) override { }

  virtual void enterOC_ReturnBody(CypherParser::OC_ReturnBodyContext * /*ctx*/) override { }
  virtual void exitOC_ReturnBody(CypherParser::OC_ReturnBodyContext * /*ctx*/) override { }

  virtual void enterOC_ReturnItems(CypherParser::OC_ReturnItemsContext * /*ctx*/) override { }
  virtual void exitOC_ReturnItems(CypherParser::OC_ReturnItemsContext * /*ctx*/) override { }

  virtual void enterOC_ReturnItem(CypherParser::OC_ReturnItemContext * /*ctx*/) override { }
  virtual void exitOC_ReturnItem(CypherParser::OC_ReturnItemContext * /*ctx*/) override { }

  virtual void enterOC_Order(CypherParser::OC_OrderContext * /*ctx*/) override { }
  virtual void exitOC_Order(CypherParser::OC_OrderContext * /*ctx*/) override { }

  virtual void enterOC_Skip(CypherParser::OC_SkipContext * /*ctx*/) override { }
  virtual void exitOC_Skip(CypherParser::OC_SkipContext * /*ctx*/) override { }

  virtual void enterOC_Limit(CypherParser::OC_LimitContext * /*ctx*/) override { }
  virtual void exitOC_Limit(CypherParser::OC_LimitContext * /*ctx*/) override { }

  virtual void enterOC_SortItem(CypherParser::OC_SortItemContext * /*ctx*/) override { }
  virtual void exitOC_SortItem(CypherParser::OC_SortItemContext * /*ctx*/) override { }

  virtual void enterOC_Where(CypherParser::OC_WhereContext * /*ctx*/) override { }
  virtual void exitOC_Where(CypherParser::OC_WhereContext * /*ctx*/) override { }

  virtual void enterOC_Pattern(CypherParser::OC_PatternContext * /*ctx*/) override { }
  virtual void exitOC_Pattern(CypherParser::OC_PatternContext * /*ctx*/) override { }

  virtual void enterOC_PatternPart(CypherParser::OC_PatternPartContext * /*ctx*/) override { }
  virtual void exitOC_PatternPart(CypherParser::OC_PatternPartContext * /*ctx*/) override { }

  virtual void enterOC_AnonymousPatternPart(CypherParser::OC_AnonymousPatternPartContext * /*ctx*/) override { }
  virtual void exitOC_AnonymousPatternPart(CypherParser::OC_AnonymousPatternPartContext * /*ctx*/) override { }

  virtual void enterOC_PatternElement(CypherParser::OC_PatternElementContext * /*ctx*/) override { }
  virtual void exitOC_PatternElement(CypherParser::OC_PatternElementContext * /*ctx*/) override { }

  virtual void enterOC_NodePattern(CypherParser::OC_NodePatternContext * /*ctx*/) override { }
  virtual void exitOC_NodePattern(CypherParser::OC_NodePatternContext * /*ctx*/) override { }

  virtual void enterOC_PatternElementChain(CypherParser::OC_PatternElementChainContext * /*ctx*/) override { }
  virtual void exitOC_PatternElementChain(CypherParser::OC_PatternElementChainContext * /*ctx*/) override { }

  virtual void enterOC_RelationshipPattern(CypherParser::OC_RelationshipPatternContext * /*ctx*/) override { }
  virtual void exitOC_RelationshipPattern(CypherParser::OC_RelationshipPatternContext * /*ctx*/) override { }

  virtual void enterOC_RelationshipDetail(CypherParser::OC_RelationshipDetailContext * /*ctx*/) override { }
  virtual void exitOC_RelationshipDetail(CypherParser::OC_RelationshipDetailContext * /*ctx*/) override { }

  virtual void enterOC_Properties(CypherParser::OC_PropertiesContext * /*ctx*/) override { }
  virtual void exitOC_Properties(CypherParser::OC_PropertiesContext * /*ctx*/) override { }

  virtual void enterOC_RelationshipTypes(CypherParser::OC_RelationshipTypesContext * /*ctx*/) override { }
  virtual void exitOC_RelationshipTypes(CypherParser::OC_RelationshipTypesContext * /*ctx*/) override { }

  virtual void enterOC_NodeLabels(CypherParser::OC_NodeLabelsContext * /*ctx*/) override { }
  virtual void exitOC_NodeLabels(CypherParser::OC_NodeLabelsContext * /*ctx*/) override { }

  virtual void enterOC_NodeLabel(CypherParser::OC_NodeLabelContext * /*ctx*/) override { }
  virtual void exitOC_NodeLabel(CypherParser::OC_NodeLabelContext * /*ctx*/) override { }

  virtual void enterOC_RangeLiteral(CypherParser::OC_RangeLiteralContext * /*ctx*/) override { }
  virtual void exitOC_RangeLiteral(CypherParser::OC_RangeLiteralContext * /*ctx*/) override { }

  virtual void enterOC_LabelName(CypherParser::OC_LabelNameContext * /*ctx*/) override { }
  virtual void exitOC_LabelName(CypherParser::OC_LabelNameContext * /*ctx*/) override { }

  virtual void enterOC_RelTypeName(CypherParser::OC_RelTypeNameContext * /*ctx*/) override { }
  virtual void exitOC_RelTypeName(CypherParser::OC_RelTypeNameContext * /*ctx*/) override { }

  virtual void enterOC_Expression(CypherParser::OC_ExpressionContext * /*ctx*/) override { }
  virtual void exitOC_Expression(CypherParser::OC_ExpressionContext * /*ctx*/) override { }

  virtual void enterOC_OrExpression(CypherParser::OC_OrExpressionContext * /*ctx*/) override { }
  virtual void exitOC_OrExpression(CypherParser::OC_OrExpressionContext * /*ctx*/) override { }

  virtual void enterOC_XorExpression(CypherParser::OC_XorExpressionContext * /*ctx*/) override { }
  virtual void exitOC_XorExpression(CypherParser::OC_XorExpressionContext * /*ctx*/) override { }

  virtual void enterOC_AndExpression(CypherParser::OC_AndExpressionContext * /*ctx*/) override { }
  virtual void exitOC_AndExpression(CypherParser::OC_AndExpressionContext * /*ctx*/) override { }

  virtual void enterOC_NotExpression(CypherParser::OC_NotExpressionContext * /*ctx*/) override { }
  virtual void exitOC_NotExpression(CypherParser::OC_NotExpressionContext * /*ctx*/) override { }

  virtual void enterOC_ComparisonExpression(CypherParser::OC_ComparisonExpressionContext * /*ctx*/) override { }
  virtual void exitOC_ComparisonExpression(CypherParser::OC_ComparisonExpressionContext * /*ctx*/) override { }

  virtual void enterOC_AddOrSubtractExpression(CypherParser::OC_AddOrSubtractExpressionContext * /*ctx*/) override { }
  virtual void exitOC_AddOrSubtractExpression(CypherParser::OC_AddOrSubtractExpressionContext * /*ctx*/) override { }

  virtual void enterOC_MultiplyDivideModuloExpression(CypherParser::OC_MultiplyDivideModuloExpressionContext * /*ctx*/) override { }
  virtual void exitOC_MultiplyDivideModuloExpression(CypherParser::OC_MultiplyDivideModuloExpressionContext * /*ctx*/) override { }

  virtual void enterOC_PowerOfExpression(CypherParser::OC_PowerOfExpressionContext * /*ctx*/) override { }
  virtual void exitOC_PowerOfExpression(CypherParser::OC_PowerOfExpressionContext * /*ctx*/) override { }

  virtual void enterOC_UnaryAddOrSubtractExpression(CypherParser::OC_UnaryAddOrSubtractExpressionContext * /*ctx*/) override { }
  virtual void exitOC_UnaryAddOrSubtractExpression(CypherParser::OC_UnaryAddOrSubtractExpressionContext * /*ctx*/) override { }

  virtual void enterOC_StringListNullOperatorExpression(CypherParser::OC_StringListNullOperatorExpressionContext * /*ctx*/) override { }
  virtual void exitOC_StringListNullOperatorExpression(CypherParser::OC_StringListNullOperatorExpressionContext * /*ctx*/) override { }

  virtual void enterOC_ListOperatorExpression(CypherParser::OC_ListOperatorExpressionContext * /*ctx*/) override { }
  virtual void exitOC_ListOperatorExpression(CypherParser::OC_ListOperatorExpressionContext * /*ctx*/) override { }

  virtual void enterOC_StringOperatorExpression(CypherParser::OC_StringOperatorExpressionContext * /*ctx*/) override { }
  virtual void exitOC_StringOperatorExpression(CypherParser::OC_StringOperatorExpressionContext * /*ctx*/) override { }

  virtual void enterOC_NullOperatorExpression(CypherParser::OC_NullOperatorExpressionContext * /*ctx*/) override { }
  virtual void exitOC_NullOperatorExpression(CypherParser::OC_NullOperatorExpressionContext * /*ctx*/) override { }

  virtual void enterOC_PropertyOrLabelsExpression(CypherParser::OC_PropertyOrLabelsExpressionContext * /*ctx*/) override { }
  virtual void exitOC_PropertyOrLabelsExpression(CypherParser::OC_PropertyOrLabelsExpressionContext * /*ctx*/) override { }

  virtual void enterOC_Atom(CypherParser::OC_AtomContext * /*ctx*/) override { }
  virtual void exitOC_Atom(CypherParser::OC_AtomContext * /*ctx*/) override { }

  virtual void enterOC_Literal(CypherParser::OC_LiteralContext * /*ctx*/) override { }
  virtual void exitOC_Literal(CypherParser::OC_LiteralContext * /*ctx*/) override { }

  virtual void enterOC_BooleanLiteral(CypherParser::OC_BooleanLiteralContext * /*ctx*/) override { }
  virtual void exitOC_BooleanLiteral(CypherParser::OC_BooleanLiteralContext * /*ctx*/) override { }

  virtual void enterOC_ListLiteral(CypherParser::OC_ListLiteralContext * /*ctx*/) override { }
  virtual void exitOC_ListLiteral(CypherParser::OC_ListLiteralContext * /*ctx*/) override { }

  virtual void enterOC_PartialComparisonExpression(CypherParser::OC_PartialComparisonExpressionContext * /*ctx*/) override { }
  virtual void exitOC_PartialComparisonExpression(CypherParser::OC_PartialComparisonExpressionContext * /*ctx*/) override { }

  virtual void enterOC_ParenthesizedExpression(CypherParser::OC_ParenthesizedExpressionContext * /*ctx*/) override { }
  virtual void exitOC_ParenthesizedExpression(CypherParser::OC_ParenthesizedExpressionContext * /*ctx*/) override { }

  virtual void enterOC_RelationshipsPattern(CypherParser::OC_RelationshipsPatternContext * /*ctx*/) override { }
  virtual void exitOC_RelationshipsPattern(CypherParser::OC_RelationshipsPatternContext * /*ctx*/) override { }

  virtual void enterOC_FilterExpression(CypherParser::OC_FilterExpressionContext * /*ctx*/) override { }
  virtual void exitOC_FilterExpression(CypherParser::OC_FilterExpressionContext * /*ctx*/) override { }

  virtual void enterOC_IdInColl(CypherParser::OC_IdInCollContext * /*ctx*/) override { }
  virtual void exitOC_IdInColl(CypherParser::OC_IdInCollContext * /*ctx*/) override { }

  virtual void enterOC_FunctionInvocation(CypherParser::OC_FunctionInvocationContext * /*ctx*/) override { }
  virtual void exitOC_FunctionInvocation(CypherParser::OC_FunctionInvocationContext * /*ctx*/) override { }

  virtual void enterOC_FunctionName(CypherParser::OC_FunctionNameContext * /*ctx*/) override { }
  virtual void exitOC_FunctionName(CypherParser::OC_FunctionNameContext * /*ctx*/) override { }

  virtual void enterOC_ExplicitProcedureInvocation(CypherParser::OC_ExplicitProcedureInvocationContext * /*ctx*/) override { }
  virtual void exitOC_ExplicitProcedureInvocation(CypherParser::OC_ExplicitProcedureInvocationContext * /*ctx*/) override { }

  virtual void enterOC_ImplicitProcedureInvocation(CypherParser::OC_ImplicitProcedureInvocationContext * /*ctx*/) override { }
  virtual void exitOC_ImplicitProcedureInvocation(CypherParser::OC_ImplicitProcedureInvocationContext * /*ctx*/) override { }

  virtual void enterOC_ProcedureResultField(CypherParser::OC_ProcedureResultFieldContext * /*ctx*/) override { }
  virtual void exitOC_ProcedureResultField(CypherParser::OC_ProcedureResultFieldContext * /*ctx*/) override { }

  virtual void enterOC_ProcedureName(CypherParser::OC_ProcedureNameContext * /*ctx*/) override { }
  virtual void exitOC_ProcedureName(CypherParser::OC_ProcedureNameContext * /*ctx*/) override { }

  virtual void enterOC_Namespace(CypherParser::OC_NamespaceContext * /*ctx*/) override { }
  virtual void exitOC_Namespace(CypherParser::OC_NamespaceContext * /*ctx*/) override { }

  virtual void enterOC_ListComprehension(CypherParser::OC_ListComprehensionContext * /*ctx*/) override { }
  virtual void exitOC_ListComprehension(CypherParser::OC_ListComprehensionContext * /*ctx*/) override { }

  virtual void enterOC_PatternComprehension(CypherParser::OC_PatternComprehensionContext * /*ctx*/) override { }
  virtual void exitOC_PatternComprehension(CypherParser::OC_PatternComprehensionContext * /*ctx*/) override { }

  virtual void enterOC_PropertyLookup(CypherParser::OC_PropertyLookupContext * /*ctx*/) override { }
  virtual void exitOC_PropertyLookup(CypherParser::OC_PropertyLookupContext * /*ctx*/) override { }

  virtual void enterOC_CaseExpression(CypherParser::OC_CaseExpressionContext * /*ctx*/) override { }
  virtual void exitOC_CaseExpression(CypherParser::OC_CaseExpressionContext * /*ctx*/) override { }

  virtual void enterOC_CaseAlternatives(CypherParser::OC_CaseAlternativesContext * /*ctx*/) override { }
  virtual void exitOC_CaseAlternatives(CypherParser::OC_CaseAlternativesContext * /*ctx*/) override { }

  virtual void enterOC_Variable(CypherParser::OC_VariableContext * /*ctx*/) override { }
  virtual void exitOC_Variable(CypherParser::OC_VariableContext * /*ctx*/) override { }

  virtual void enterOC_NumberLiteral(CypherParser::OC_NumberLiteralContext * /*ctx*/) override { }
  virtual void exitOC_NumberLiteral(CypherParser::OC_NumberLiteralContext * /*ctx*/) override { }

  virtual void enterOC_MapLiteral(CypherParser::OC_MapLiteralContext * /*ctx*/) override { }
  virtual void exitOC_MapLiteral(CypherParser::OC_MapLiteralContext * /*ctx*/) override { }

  virtual void enterOC_Parameter(CypherParser::OC_ParameterContext * /*ctx*/) override { }
  virtual void exitOC_Parameter(CypherParser::OC_ParameterContext * /*ctx*/) override { }

  virtual void enterOC_PropertyExpression(CypherParser::OC_PropertyExpressionContext * /*ctx*/) override { }
  virtual void exitOC_PropertyExpression(CypherParser::OC_PropertyExpressionContext * /*ctx*/) override { }

  virtual void enterOC_PropertyKeyName(CypherParser::OC_PropertyKeyNameContext * /*ctx*/) override { }
  virtual void exitOC_PropertyKeyName(CypherParser::OC_PropertyKeyNameContext * /*ctx*/) override { }

  virtual void enterOC_IntegerLiteral(CypherParser::OC_IntegerLiteralContext * /*ctx*/) override { }
  virtual void exitOC_IntegerLiteral(CypherParser::OC_IntegerLiteralContext * /*ctx*/) override { }

  virtual void enterOC_DoubleLiteral(CypherParser::OC_DoubleLiteralContext * /*ctx*/) override { }
  virtual void exitOC_DoubleLiteral(CypherParser::OC_DoubleLiteralContext * /*ctx*/) override { }

  virtual void enterOC_SchemaName(CypherParser::OC_SchemaNameContext * /*ctx*/) override { }
  virtual void exitOC_SchemaName(CypherParser::OC_SchemaNameContext * /*ctx*/) override { }

  virtual void enterOC_ReservedWord(CypherParser::OC_ReservedWordContext * /*ctx*/) override { }
  virtual void exitOC_ReservedWord(CypherParser::OC_ReservedWordContext * /*ctx*/) override { }

  virtual void enterOC_SymbolicName(CypherParser::OC_SymbolicNameContext * /*ctx*/) override { }
  virtual void exitOC_SymbolicName(CypherParser::OC_SymbolicNameContext * /*ctx*/) override { }

  virtual void enterOC_LeftArrowHead(CypherParser::OC_LeftArrowHeadContext * /*ctx*/) override { }
  virtual void exitOC_LeftArrowHead(CypherParser::OC_LeftArrowHeadContext * /*ctx*/) override { }

  virtual void enterOC_RightArrowHead(CypherParser::OC_RightArrowHeadContext * /*ctx*/) override { }
  virtual void exitOC_RightArrowHead(CypherParser::OC_RightArrowHeadContext * /*ctx*/) override { }

  virtual void enterOC_Dash(CypherParser::OC_DashContext * /*ctx*/) override { }
  virtual void exitOC_Dash(CypherParser::OC_DashContext * /*ctx*/) override { }


  virtual void enterEveryRule(antlr4::ParserRuleContext * /*ctx*/) override { }
  virtual void exitEveryRule(antlr4::ParserRuleContext * /*ctx*/) override { }
  virtual void visitTerminal(antlr4::tree::TerminalNode * /*node*/) override { }
  virtual void visitErrorNode(antlr4::tree::ErrorNode * /*node*/) override { }

};

