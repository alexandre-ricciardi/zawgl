// CypherEngine.cpp : Définit les fonctions exportées pour l'application DLL.
//

#include "CypherEngine.h"
#include <iostream>
#include "CypherEngineImpl.h"

CypherEngine::CypherEngine()
{

}

void CypherEngine::process(const std::string & expr)
{
	CypherEngineImpl impl;
	impl.process(expr);
}

