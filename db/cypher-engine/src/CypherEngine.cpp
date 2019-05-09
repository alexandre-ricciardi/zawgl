// CypherEngine.cpp : Définit les fonctions exportées pour l'application DLL.
//

#include "CypherEngine.h"
#include <iostream>
#include "CypherEngineImpl.h"

CypherEngine::CypherEngine(GraphRepository& gr): gr(gr)
{

}

void CypherEngine::process(const std::string & expr)
{
	CypherEngineImpl impl(gr);
	impl.process(expr);
}

